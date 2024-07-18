pub mod command;

use std::path::Path;

use ab_glyph::{FontVec, PxScale};
use command::{settings::settings, version::version};
use image::{imageops::FilterType, Rgba};
use img_gen::{error::Error, ImageBuilder, ImageGenerator};
use log::{info, warn};
use poise::serenity_prelude::{self as serenity, CreateAttachment, CreateMessage};
use tempdir::TempDir;
use tokio::{fs::File, io::AsyncWriteExt};

type PoiseError = Box<dyn std::error::Error + Send + Sync>;

pub static FIRA_SANS_BOLD: &str = "fsb";
pub static FIRA_MONO_MEDIUM: &str = "fmm";
static BACK_BANNER_PATH: &str = "assets/userbanner_back.png";
static FRONT_BANNER_PATH: &str = "assets/userbanner.png";

pub struct Data {
    image_generator: ImageGenerator,
    temp_dir: TempDir,
}

fn setup_image_generator() -> Result<ImageGenerator, Error> {
    let fira_sans_bold =
        FontVec::try_from_vec(include_bytes!("../assets/FiraSans-Bold.ttf").to_vec())?;
    let fira_mono_medium =
        FontVec::try_from_vec(include_bytes!("../assets/FiraMono-Medium.ttf").to_vec())?;

    let mut img_generator = ImageGenerator::new();
    img_generator.add_font(FIRA_SANS_BOLD, fira_sans_bold);
    img_generator.add_font(FIRA_MONO_MEDIUM, fira_mono_medium);

    Ok(img_generator)
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, PoiseError>,
    data: &Data,
) -> Result<(), PoiseError> {
    if let serenity::FullEvent::GuildMemberAddition { new_member } = event {
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

        let (x, y) = (322, 64);
        let big_scale = PxScale { x: 40., y: 40. };
        let smollscale = PxScale { x: 24., y: 24. };

        let guild = ctx.http.get_guild(new_member.guild_id).await?;
        let members = guild.members(&ctx.http, None, None).await?.len();

        let image_builder = get_image_builder(
            file_path,
            x,
            y,
            new_member.display_name(),
            members,
            big_scale,
            smollscale,
        );
        let output_image = data.image_generator.generate(image_builder)?;

        let outfile_id = uuid::Uuid::new_v4();
        let outfile_path = data.temp_dir.path().join(format!("{}.png", outfile_id));
        output_image.save(&outfile_path)?;

        if let Some(system_channel_id) = guild.system_channel_id {
            let attachment = CreateAttachment::path(outfile_path).await?;
            let message = CreateMessage::new()
                .content(format!(
                    "Hey <@{}>, welcome to **{}**",
                    new_member.user.id, guild.name
                ))
                .add_file(attachment);

            system_channel_id.send_message(&ctx.http, message).await?;
        }
    }

    Ok(())
}

fn get_image_builder<T: AsRef<Path>>(
    file_path: T,
    x: i64,
    y: i64,
    display_name: &str,
    members: usize,
    big_scale: PxScale,
    small_scale: PxScale,
) -> ImageBuilder {
    let image_builder = ImageBuilder::new(BACK_BANNER_PATH)
        .add_image(&file_path, x, y)
        .add_image(FRONT_BANNER_PATH, 0, 0)
        .add_text(
            &format!("{} just joined the server", display_name),
            450,
            352,
            big_scale,
            FIRA_SANS_BOLD,
            Rgba([34, 34, 34, 255]),
            true,
        )
        .add_text(
            &format!("You are the #{} member", members),
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let tmp_dir = TempDir::new("welcomebot").expect("Tempdir could not be created");

    let img_generator = setup_image_generator()?;

    dotenvy::dotenv().ok();
    let token = std::env::var("WELCOMEBOT_TOKEN").expect("Missing WELCOMEBOT_TOKEN.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![version(), settings()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    image_generator: img_generator,
                    temp_dir: tmp_dir,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

    Ok(())
}
