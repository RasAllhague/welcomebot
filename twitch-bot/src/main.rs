mod bot;
mod builder;
mod utils;
mod error;

use builder::TtvBotBuilder;
use corelib::logging::setup_observability;
use error::Error;
use migration::{Migrator, MigratorTrait, sea_orm::Database};
use serenity::{Client, model::prelude::*};
use ttv::bot::TwitchBot;
use twitch_api::types::UserName;
use twitch_oauth2::{ClientId, Scope};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _guard = setup_observability("twitch_bot");
    log::info!("Starting twitch bot...");

    dotenvy::dotenv().ok();

    let token = std::env::var("WELCOMEBOT_TOKEN").expect("Missing WELCOMEBOT_TOKEN.");
    let db_url = std::env::var("WELCOME_DATABASE_URL")
        .expect("WELCOME_DATABASE_URL is not set in .env file");
    let twitch_client_id =
        std::env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID is not set in .env file");
    let twitch_bot_login = std::env::var("TWITCH_BOT_LOGIN")
        .map(|x| UserName::new(x))
        .expect("TWITCH_BOT_LOGIN is not set in .env file");
    let twitch_client_secret = std::env::var("TWITCH_CLIENT_SECRET")
        .expect("TWITCH_CLIENT_SECRET is not set in .env file");
    let redirect_url = std::env::var("REDIRECT_URL")
        .map(|s| Url::parse(&s))
        .expect("REDIRECT_URL is not set in .env file")
        .expect("REDIRECT_URL is not in a valid format");
    let twitch_logins =
        std::env::var("BROADCASTER_LOGINS").expect("BROADCASTER_LOGINS is not set in .env file");

    // Connect to the database
    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to open db connection.");
    Migrator::up(&conn, None)
        .await
        .expect("Failed to run migrations.");

    let twitch_client_id = ClientId::new(twitch_client_id);

    // Set up Discord gateway intents
    let discord_intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS;
    let discord_client = Client::builder(token, discord_intents).await.unwrap();

    // Set up the Twitch bot
    let ttv_bot = TtvBotBuilder::new(&conn, twitch_client_id.clone(), twitch_bot_login)
        .set_authorization_code_flow(
            twitch_client_id,
            twitch_client_secret.into(),
            vec![
                Scope::ChannelModerate,
                Scope::UserReadChat,
                Scope::ModeratorReadWarnings,
            ],
            redirect_url,
        )
        .add_broadcaster_login(twitch_logins.into())
        .build()
        .await?;

    ttv_bot.start().await?;

    // Flush logs
    fastrace::flush();

    Ok(())
}
