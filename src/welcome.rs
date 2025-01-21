use std::path::Path;

use ab_glyph::{FontVec, PxScale};
use image::{imageops::FilterType, Rgba};
use img_gen::{error::Error, ImageBuilder, ImageGenerator};
use log::{info, warn};
use poise::serenity_prelude::{self as serenity, ChannelId, CreateAttachment, CreateMessage};
use tempfile::TempDir;
use tokio::{fs::File, io::AsyncWriteExt};
use welcome_service::{guild_query, image_query, welcome_settings_query};

use crate::{Data, PoiseError};

static FIRA_SANS_BOLD: &str = "fsb";
static FIRA_MONO_MEDIUM: &str = "fmm";
const FIRA_SANS_BOLD_FILE: &[u8] = include_bytes!("../assets/FiraSans-Bold.ttf");
const FIRA_MONO_MEDIUM_FILE: &[u8] = include_bytes!("../assets/FiraMono-Medium.ttf");

const IMAGE_X: i64 = 322;
const IMAGE_Y: i64 = 64;
const BIG_SCALE: PxScale = PxScale { x: 40., y: 40. };
const SMALL_SCALE: PxScale = PxScale { x: 24., y: 24. };

pub fn setup_image_generator() -> Result<ImageGenerator, Error> {
    let fira_sans_bold = FontVec::try_from_vec(FIRA_SANS_BOLD_FILE.to_vec())?;
    let fira_mono_medium = FontVec::try_from_vec(FIRA_MONO_MEDIUM_FILE.to_vec())?;

    let mut img_generator = ImageGenerator::new();
    img_generator.add_font(FIRA_SANS_BOLD, fira_sans_bold);
    img_generator.add_font(FIRA_MONO_MEDIUM, fira_mono_medium);

    Ok(img_generator)
}

fn create_image_builder(
    front_image_path: impl AsRef<Path>,
    back_image_path: impl AsRef<Path>,
    file_path: impl AsRef<Path>,
    headline_message: impl AsRef<str>,
    subline_message: impl AsRef<str>,
    x: i64,
    y: i64,
    display_name: impl AsRef<str>,
    members: usize,
    big_scale: PxScale,
    small_scale: PxScale,
) -> ImageBuilder {
    let image_builder = ImageBuilder::new(back_image_path)
        .add_image(&file_path, x, y)
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

async fn download_avatar(
    img_url: &str,
    temp_dir: &TempDir,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error + Sync + Send>> {
    let image_bytes = reqwest::get(img_url).await?.bytes().await?;

    let image_id = uuid::Uuid::new_v4();

    let file_path = if img_url.to_lowercase().contains(".webp") {
        temp_dir.path().join(format!("{}.webp", image_id))
    } else {
        temp_dir.path().join(format!("{}.png", image_id))
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

pub async fn send_welcome_message(
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

    let mut img_url = new_member
        .avatar_url()
        .or(new_member.user.avatar_url())
        .unwrap_or(new_member.user.default_avatar_url());

    info!("Img url: {}", img_url);

    if img_url.contains(".png") {
        img_url = new_member.user.default_avatar_url();
    }

    let file_path = download_avatar(&img_url, &data.temp_dir).await?;

    let guild = ctx.http.get_guild(new_member.guild_id).await?;
    let members = guild.members(&ctx.http, None, None).await?.len();

    let db = &data.conn;

    if let Some(guild_model) = guild_query::get_by_guild_id(db, guild.id.into()).await? {
        if let Some(settings_id) = guild_model.welcome_settings_id {
            let welcome_settings = match welcome_settings_query::get_one(db, settings_id).await? {
                Some(m) => m,
                None => return Ok(()),
            };

            if !welcome_settings.enabled {
                return Ok(());
            }

            let back_image_model =
                match image_query::get_one(db, welcome_settings.back_banner).await? {
                    Some(m) => m,
                    None => return Ok(()),
                };
            let front_image_model =
                match image_query::get_one(db, welcome_settings.front_banner).await? {
                    Some(m) => m,
                    None => return Ok(()),
                };

            let image_builder = create_image_builder(
                back_image_model.path,
                front_image_model.path,
                file_path,
                &welcome_settings.image_headline,
                &welcome_settings.image_subtext,
                IMAGE_X,
                IMAGE_Y,
                new_member.display_name(),
                members,
                BIG_SCALE,
                SMALL_SCALE,
            );
            let output_image = data.image_generator.generate(image_builder)?;

            let outfile_id = uuid::Uuid::new_v4();
            let outfile_path = data.temp_dir.path().join(format!("{}.png", outfile_id));
            output_image.save(&outfile_path)?;

            let channel = ChannelId::new(welcome_settings.welcome_channel as u64);

            let message = welcome_settings
                .chat_message
                .replace("{user}", &format!("<@{}>", new_member.user.id))
                .replace("{guild_name}", &guild.name);

            let attachment = CreateAttachment::path(outfile_path).await?;
            let message = CreateMessage::new().content(message).add_file(attachment);

            channel.send_message(&ctx.http, message).await?;
        }
    }
    Ok(())
}
