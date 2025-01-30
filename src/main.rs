pub mod command;
mod embed;
pub mod error;
mod moderation;
pub mod util;
mod welcome;

use command::{moderation::moderation, version::version, welcome::welcome};
use img_gen::{error::Error, ImageGenerator};
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use moderation::{ban_suspicious_user, update_ban_log};
use poise::serenity_prelude::{self as serenity};
use tempfile::{tempdir, TempDir};
use welcome::{handle_member_join, setup_image_generator};

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
            handle_member_join(ctx, data, new_member).await
        }
        serenity::FullEvent::GuildMemberUpdate {
            old_if_available: _,
            new,
            event,
        } => ban_suspicious_user(ctx, data, new.as_ref(), event).await,
        serenity::FullEvent::GuildBanAddition {
            guild_id,
            banned_user,
        } => update_ban_log(ctx, data, guild_id, banned_user, framework.bot_id.into()).await,
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
            commands: vec![version(), welcome(), moderation()],
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
