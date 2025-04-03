pub mod command;
mod embed;
pub mod error;
pub mod interaction;
mod moderation;
pub mod util;
mod welcome;

use command::{moderation::moderation, version::version, welcome::welcome};
use crossbeam_channel::Receiver;
use error::Error;
use fastrace::collector::{Config, ConsoleReporter};
use futures::TryFutureExt;
use img_gen::ImageGenerator;
use logforth::{
    append::{
        rolling_file::{self, RollingFileWriter, Rotation},
        RollingFile,
    },
    diagnostic,
    layout::JsonLayout,
    non_blocking::WorkerGuard,
};
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use moderation::{handle_suspicious_user, update_ban_log};
use poise::serenity_prelude::{self as serenity};
use tempfile::{tempdir, TempDir};
use ttv::{builder::TtvBotBuilder, queue::BotEvent};
use twitch_oauth2::ClientId;
use welcome::{handle_member_join, setup_image_generator};

/// Represents the error type used throughout the bot.
pub type PoiseError = Box<dyn std::error::Error + Send + Sync>;

/// Represents the context passed to commands and event handlers.
pub type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// Represents the shared data used by the bot.
pub struct Data {
    /// The database connection.
    conn: DatabaseConnection,
    /// The image generator for creating welcome images.
    image_generator: ImageGenerator,
    /// A temporary directory for storing files.
    temp_dir: TempDir,
    /// A receiver for Twitch bot events.
    receiver: Receiver<BotEvent>,
}

/// Handles events received from Discord.
///
/// This function processes various events, such as member additions, member updates,
/// and guild bans, and performs the appropriate actions.
///
/// # Arguments
/// * `ctx` - The Serenity context.
/// * `event` - The event received from Discord.
/// * `framework` - The Poise framework context.
/// * `data` - The shared data for the bot.
///
/// # Errors
/// Returns a [`PoiseError`] if any operation fails.
#[fastrace::trace]
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
        } => handle_suspicious_user(ctx, data, new.as_ref(), event).await,
        serenity::FullEvent::GuildBanAddition {
            guild_id,
            banned_user,
        } => update_ban_log(ctx, data, guild_id, banned_user, framework.bot_id.into()).await,
        _ => Ok(()),
    }
}

/// The main entry point for the bot.
///
/// This function initializes the bot, sets up the framework, connects to the database,
/// and starts the bot's event loop.
///
/// # Errors
/// Returns an [`Error`] if any initialization or runtime operation fails.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let _guard = setup_observability();
    log::info!("Starting welcome bot...");

    // Create a temporary directory for storing files
    let tmp_dir = tempdir().expect("Tempdir could not be created");

    // Set up the image generator
    let img_generator = setup_image_generator()?;

    // Load environment variables
    dotenvy::dotenv().ok();
    let token = std::env::var("WELCOMEBOT_TOKEN").expect("Missing WELCOMEBOT_TOKEN.");
    let db_url = std::env::var("WELCOME_DATABASE_URL")
        .expect("WELCOME_DATABASE_URL is not set in .env file");
    let twitch_client_id =
        std::env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID is not set in .env file");

    // Set up Discord gateway intents
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    // Connect to the database
    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to open db connection.");
    Migrator::up(&conn, None)
        .await
        .expect("Failed to run migrations.");

    // Set up the Twitch bot
    let (ttv_bot, receiver) = TtvBotBuilder::new(&conn, ClientId::new(twitch_client_id))
        .add_broadcaster_login("rasallhague".into())
        .build()
        .await?;

    // Set up the Poise framework
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
                    receiver,
                })
            })
        })
        .build();

    // Create the Serenity client
    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    // Start the Twitch bot and the Discord client concurrently
    futures::future::try_join(
        ttv_bot.start().map_err(|x| Error::Ttv(x)),
        client.start().map_err(|x| Error::Serenity(x)),
    )
    .await?;

    // Flush logs
    fastrace::flush();

    Ok(())
}

/// Sets up observability for the bot.
///
/// This function configures logging and tracing for the bot, including rolling file logs
/// and console reporters.
///
/// # Returns
/// A `WorkerGuard` that ensures logs are flushed when the application exits.
fn setup_observability() -> WorkerGuard {
    let rolling_writer = RollingFileWriter::builder()
        .rotation(Rotation::Daily)
        .filename_prefix("app_log")
        .build("logs")
        .unwrap();

    let (non_blocking, guard) = rolling_file::non_blocking(rolling_writer).finish();

    logforth::builder()
        .dispatch(|d| {
            d.filter(log::LevelFilter::Trace)
                .append(logforth::append::FastraceEvent::default())
        })
        .dispatch(|d| {
            d.diagnostic(diagnostic::FastraceDiagnostic::default())
                .append(logforth::append::Stderr::default())
        })
        .dispatch(|d| {
            d.filter(log::LevelFilter::Trace)
                .append(RollingFile::new(non_blocking).with_layout(JsonLayout::default()))
        })
        .apply();

    fastrace::set_reporter(ConsoleReporter, Config::default());

    guard
}
