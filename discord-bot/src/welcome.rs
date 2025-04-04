use std::path::{Path, PathBuf};

use ab_glyph::{FontVec, PxScale};
use entity::welcome_settings;
use image::{Rgba, imageops::FilterType};
use img_gen::{ImageBuilder, ImageGenerator, Vec2, error::Error};
use log::{info, warn};
use migration::{DbErr, sea_orm::DbConn};
use poise::serenity_prelude::{self as serenity, ChannelId, CreateAttachment, CreateMessage};
use tempfile::TempDir;
use tokio::{fs::File, io::AsyncWriteExt};
use welcome_service::{guild_query, image_query, welcome_settings_query};

use crate::{Data, PoiseError, moderation::send_suspicious_user_embed};

static FIRA_SANS_BOLD: &str = "fsb";
static FIRA_MONO_MEDIUM: &str = "fmm";
const FIRA_SANS_BOLD_FILE: &[u8] = include_bytes!("../assets/FiraSans-Bold.ttf");
const FIRA_MONO_MEDIUM_FILE: &[u8] = include_bytes!("../assets/FiraMono-Medium.ttf");

const IMAGE_POSITION: Vec2<i64> = Vec2::<i64>::new(322, 64);
const BIG_SCALE: PxScale = PxScale { x: 40., y: 40. };
const SMALL_SCALE: PxScale = PxScale { x: 24., y: 24. };

/// Represents the context for generating welcome images.
#[derive(Debug, Clone)]
pub struct ImageContext {
    /// The path to the background image.
    pub back_image: PathBuf,
    /// The path to the foreground image.
    pub front_image: PathBuf,
    /// The headline message to display on the image.
    pub headline_message: String,
    /// The subline message to display on the image.
    pub subline_message: String,
}

impl ImageContext {
    /// Initializes the `ImageContext` from the database and welcome settings.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `welcome_settings` - The welcome settings model.
    ///
    /// # Returns
    /// Returns an `Option` containing the `ImageContext` if successful, or `None` if required data is missing.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if any database operation fails.
    #[fastrace::trace]
    pub async fn init(
        db: &DbConn,
        welcome_settings: &welcome_settings::Model,
    ) -> Result<Option<Self>, DbErr> {
        let Some(back_image_model) = image_query::get_one(db, welcome_settings.back_banner).await?
        else {
            return Ok(None);
        };
        let Some(front_image_model) =
            image_query::get_one(db, welcome_settings.front_banner).await?
        else {
            return Ok(None);
        };

        Ok(Some(Self {
            back_image: PathBuf::from(back_image_model.path),
            front_image: PathBuf::from(front_image_model.path),
            headline_message: welcome_settings.image_headline.clone(),
            subline_message: welcome_settings.image_subtext.clone(),
        }))
    }
}

/// Sets up the image generator by loading fonts.
///
/// # Returns
/// Returns an `ImageGenerator` instance if successful.
///
/// # Errors
/// Returns an [`Error`] if loading fonts fails.
#[fastrace::trace]
pub fn setup_image_generator() -> Result<ImageGenerator, Error> {
    let fira_sans_bold = FontVec::try_from_vec(FIRA_SANS_BOLD_FILE.to_vec())?;
    let fira_mono_medium = FontVec::try_from_vec(FIRA_MONO_MEDIUM_FILE.to_vec())?;

    let mut img_generator = ImageGenerator::new();
    img_generator.add_font(FIRA_SANS_BOLD, fira_sans_bold);
    img_generator.add_font(FIRA_MONO_MEDIUM, fira_mono_medium);

    Ok(img_generator)
}

/// Creates an `ImageBuilder` for generating welcome images.
///
/// # Arguments
/// * `front_image_path` - The path to the foreground image.
/// * `back_image_path` - The path to the background image.
/// * `file_path` - The path to the user's avatar image.
/// * `headline_message` - The headline message to display.
/// * `subline_message` - The subline message to display.
/// * `position` - The position of the avatar on the image.
/// * `display_name` - The display name of the user.
/// * `members` - The number of members in the guild.
/// * `big_scale` - The font scale for the headline.
/// * `small_scale` - The font scale for the subline.
///
/// # Returns
/// Returns an `ImageBuilder` instance.
#[fastrace::trace]
fn create_image_builder(
    front_image_path: impl AsRef<Path>,
    back_image_path: impl AsRef<Path>,
    file_path: impl AsRef<Path>,
    headline_message: impl AsRef<str>,
    subline_message: impl AsRef<str>,
    position: Vec2<i64>,
    display_name: impl AsRef<str>,
    members: usize,
    big_scale: PxScale,
    small_scale: PxScale,
) -> ImageBuilder {
    let image_builder = ImageBuilder::new(back_image_path)
        .add_image(&file_path, position.x, position.y)
        .add_image(front_image_path, 0, 0)
        .add_text(
            &headline_message
                .as_ref()
                .replace("{name}", display_name.as_ref()),
            450,
            352,
            big_scale,
            FIRA_SANS_BOLD,
            Rgba([34, 34, 34, 255]),
            true,
        )
        .add_text(
            &subline_message
                .as_ref()
                .replace("{members}", &members.to_string()),
            450,
            400,
            small_scale,
            FIRA_MONO_MEDIUM,
            Rgba([59, 59, 59, 255]),
            true,
        );
    image_builder
}

/// Downloads and processes a user's avatar image.
///
/// # Arguments
/// * `img_url` - The URL of the user's avatar.
/// * `temp_dir` - The temporary directory to store the image.
///
/// # Returns
/// Returns the path to the processed avatar image.
///
/// # Errors
/// Returns an error if downloading or processing the image fails.
#[fastrace::trace]
async fn download_avatar(
    img_url: &str,
    temp_dir: &TempDir,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Sync + Send>> {
    let image_bytes = reqwest::get(img_url).await?.bytes().await?;

    let image_id = uuid::Uuid::new_v4();

    let file_path = if img_url.to_lowercase().contains(".webp") {
        temp_dir.path().join(format!("{image_id}.webp"))
    } else {
        temp_dir.path().join(format!("{image_id}.png"))
    };

    let mut tmp_file = File::create(&file_path).await?;
    tmp_file.write_all(&image_bytes).await?;
    tmp_file.flush().await?;

    drop(tmp_file);

    let image = image::open(&file_path)?;
    let image = image.resize(256, 256, FilterType::Nearest);
    image.save(&file_path)?;

    Ok(file_path)
}

/// Handles a new member joining the guild.
///
/// This function checks if the member is a bot, sends a suspicious user embed if necessary,
/// and sends a welcome message if welcome settings are enabled.
///
/// # Arguments
/// * `ctx` - The Serenity context.
/// * `data` - The shared bot data.
/// * `new_member` - The new member who joined the guild.
///
/// # Errors
/// Returns a [`PoiseError`] if any operation fails.
#[fastrace::trace]
pub async fn handle_member_join(
    ctx: &serenity::Context,
    data: &Data,
    new_member: &serenity::Member,
) -> Result<(), PoiseError> {
    info!(
        "User joined: Id:'{}', name:'{}'.",
        new_member.user.id,
        new_member.display_name()
    );

    if new_member.user.bot {
        warn!("Bot joined: '{}'.", new_member.display_name());
        return Ok(());
    }

    let db = &data.conn;

    let Some(guild) = guild_query::get_by_guild_id(db, new_member.guild_id.into()).await? else {
        return Ok(());
    };

    send_suspicious_user_embed(ctx, new_member, &guild).await?;

    if let Some(settings_id) = guild.welcome_settings_id {
        let Some(welcome_settings) = welcome_settings_query::get_one(db, settings_id).await? else {
            return Ok(());
        };

        if !welcome_settings.enabled {
            return Ok(());
        }

        if let Some(image_context) = ImageContext::init(db, &welcome_settings).await? {
            send_welcome_message(ctx, data, image_context, new_member, &welcome_settings).await?;
        }
    }

    Ok(())
}

/// Sends a welcome message to the specified channel.
///
/// This function generates a welcome image and sends it along with a welcome message
/// to the configured welcome channel.
///
/// # Arguments
/// * `ctx` - The Serenity context.
/// * `data` - The shared bot data.
/// * `image_context` - The context for generating the welcome image.
/// * `new_member` - The new member who joined the guild.
/// * `welcome_settings` - The welcome settings model.
///
/// # Errors
/// Returns a [`PoiseError`] if any operation fails.
#[fastrace::trace]
async fn send_welcome_message(
    ctx: &serenity::Context,
    data: &Data,
    image_context: ImageContext,
    new_member: &serenity::Member,
    welcome_settings: &welcome_settings::Model,
) -> Result<(), PoiseError> {
    let mut img_url = new_member
        .avatar_url()
        .or_else(|| new_member.user.avatar_url())
        .unwrap_or_else(|| new_member.user.default_avatar_url());

    if img_url.contains(".png") {
        img_url = new_member.user.default_avatar_url();
    }

    let partial_guild = ctx.http.get_guild(new_member.guild_id).await?;
    let members = partial_guild.members(&ctx.http, None, None).await?.len();

    let file_path = download_avatar(&img_url, &data.temp_dir).await?;
    let image_builder = create_image_builder(
        image_context.front_image,
        image_context.back_image,
        file_path,
        image_context.headline_message,
        image_context.subline_message,
        IMAGE_POSITION,
        new_member.display_name(),
        members,
        BIG_SCALE,
        SMALL_SCALE,
    );

    let output_image = data.image_generator.generate(image_builder)?;
    let outfile_id = uuid::Uuid::new_v4();
    let outfile_path = data.temp_dir.path().join(format!("{outfile_id}.png"));
    output_image.save(&outfile_path)?;

    let channel = ChannelId::new(welcome_settings.welcome_channel as u64);
    let message = welcome_settings
        .chat_message
        .replace("{user}", &format!("<@{}>", new_member.user.id))
        .replace("{guild_name}", &partial_guild.name);

    let attachment = CreateAttachment::path(outfile_path).await?;
    let message = CreateMessage::new().content(message).add_file(attachment);

    channel.send_message(&ctx.http, message).await?;
    Ok(())
}
