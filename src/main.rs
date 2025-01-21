pub mod command;
pub mod error;
mod moderation;
mod welcome;
mod embed;

use command::{settings::settings, version::version};
use img_gen::{error::Error, ImageGenerator};
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use moderation::ban_bot_user;
use poise::serenity_prelude::{self as serenity};
use tempfile::{tempdir, TempDir};
use welcome::{send_welcome_message, setup_image_generator};

pub type PoiseError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, PoiseError>;

pub struct Data {
    conn: DatabaseConnection,
    image_generator: ImageGenerator,
    temp_dir: TempDir,
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, PoiseError>,
    data: &Data,
) -> Result<(), PoiseError> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            send_welcome_message(ctx, data, new_member).await
        }
        serenity::FullEvent::GuildMemberUpdate {
            old_if_available: _,
            new,
            event,
        } => ban_bot_user(ctx, framework, data, new, event).await,
        _ => Ok(()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let tmp_dir = tempdir().expect("Tempdir could not be created");

    let img_generator = setup_image_generator()?;

    dotenvy::dotenv().ok();
    let token = std::env::var("WELCOMEBOT_TOKEN").expect("Missing WELCOMEBOT_TOKEN.");
    let db_url = std::env::var("WELCOME_DATABASE_URL")
        .expect("WELCOME_DATABASE_URL is not set in .env file");

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to open db connection.");
    Migrator::up(&conn, None)
        .await
        .expect("Failed to run migrations.");

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
                    conn,
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
