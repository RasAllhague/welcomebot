use std::sync::Arc;

use tokio::sync::Mutex;
use twitch_api::{
    eventsub::{Event, Transport},
    HelixClient,
};
use twitch_oauth2::{AccessToken, RefreshToken, TwitchToken, UserToken};

use crate::{
    error::Error,
    websocket::{self, SubscriptionIds},
};

use sea_orm::DbConn;

/// How often we should check if the token is still valid.
const TOKEN_VALIDATION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
/// The threshold at which we should refresh the token before expiration.
///
/// Only checked every [TOKEN_VALIDATION_INTERVAL] seconds
const TOKEN_EXPIRATION_THRESHOLD: std::time::Duration = std::time::Duration::from_secs(60);

pub struct TtvBotSettings {
    pub client_id: twitch_oauth2::ClientId,
    pub auth: std::path::PathBuf,
    pub config: std::path::PathBuf,
    pub broadcaster_login: Vec<twitch_api::types::UserName>,
}

pub struct TtvBot {
    settings: TtvBotSettings,
    client: HelixClient<'static, reqwest::Client>,
    token: Arc<Mutex<twitch_oauth2::UserToken>>,
    broadcasters: Vec<twitch_api::types::UserId>,
}

impl TtvBot {
    pub async fn start(&self) -> Result<(), Error> {
        let websocket = websocket::TwitchWebsocketClient {
            session_id: None,
            token: self.token.clone(),
            client: self.client.clone(),
            connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            chats: self.broadcasters.clone(),
        };

        let refresh_token = async move {
            let token = self.token.clone();
            let client = self.client.clone();

            let mut interval = tokio::time::interval(TOKEN_VALIDATION_INTERVAL);

            loop {
                interval.tick().await;
                let mut token = token.lock().await;
                refresh_and_validate_token(&mut token, &client, &self.settings).await?;
            }

            #[allow(unreachable_code)]
            Ok(())
        };

        let ws = websocket.run(
            |e, ts| async { self.handle_event(e, ts).await },
            |client, transport, token, ids| async {
                self.subscribe_events(client, transport, token, ids).await
            },
        );

        futures::future::try_join(ws, refresh_token).await?;

        Ok(())
    }

    /// Handle all eventsub events.
    /// We print the message to the console and if it's a chat message we send it to [Bot::handle_chat_message].
    /// If there's an event you want to listen to you should first add it to [websocket::ChatWebsocketClient::process_welcome_message] and then handle it here.
    async fn handle_event(
        &self,
        event: Event,
        timestamp: twitch_api::types::Timestamp,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// Handle all eventsub events.
    /// We print the message to the console and if it's a chat message we send it to [Bot::handle_chat_message].
    /// If there's an event you want to listen to you should first add it to [websocket::ChatWebsocketClient::process_welcome_message] and then handle it here.
    async fn subscribe_events(
        &self,
        client: HelixClient<'static, reqwest::Client>,
        transport: Transport,
        token: Arc<Mutex<UserToken>>,
        ids: SubscriptionIds,
    ) -> Result<(), Error> {
        Ok(())
    }
}

async fn refresh_and_validate_token(
    token: &mut UserToken,
    client: &HelixClient<'_, reqwest::Client>,
    opts: &TtvBotSettings,
) -> Result<(), Error> {
    if token.expires_in() < TOKEN_EXPIRATION_THRESHOLD {
        tracing::info!("refreshed token");
        token.refresh_token(client).await?;
        // save_token(token, &opts.auth)?;
        todo!("implement saving to db");
    }
    token.validate_token(client).await?;
    Ok(())
}

// you should probably replace this with something more robust
#[cfg(debug_assertions)]
async fn save_token_to_db(db: &DbConn, token: &twitch_oauth2::UserToken) -> Result<(), Error> {
    use entity::twitch_token::Model;
    use sea_orm::sqlx::types::chrono::Utc;
    use welcome_service::twitch_token_mutation;

    let token = Model {
        id: 0,
        access_token: Some(token.access_token.to_string()),
        refresh_token: token.refresh_token.clone().map(|x| x.to_string()),
        last_refreshed: Some(Utc::now()),
    };

    twitch_token_mutation::create_or_update(db, token).await?;

    Ok(())
}

#[cfg(debug_assertions)]
async fn load_token_from_db(
    db: &DbConn,
    client: &HelixClient<'_, reqwest::Client>,
) -> Result<Option<twitch_oauth2::UserToken>, Error> {
    use welcome_service::twitch_token_query;

    match twitch_token_query::get(db).await? {
        Some(token) => {
            let Some(access_token) = token.access_token.map(|x| AccessToken::new(x)) else {
                return Ok(None);
            };
            let refresh_token = token.refresh_token.map(|x| RefreshToken::new(x));

            let token =
                twitch_oauth2::UserToken::from_existing(client, access_token, refresh_token, None)
                    .await?;

            Ok(Some(token))
        }
        _ => Ok(None),
    }
}
