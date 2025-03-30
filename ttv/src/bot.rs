use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use twitch_api::{
    eventsub::{Event, Transport},
    HelixClient,
};
use twitch_oauth2::{TwitchToken, UserToken};

use crate::{
    error::Error,
    websocket::{self, SubscriptionIds},
};

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
        save_token(token, &opts.auth)?;
    }
    token.validate_token(client).await?;
    Ok(())
}

/// Used to save the token to a file
#[derive(Serialize, Deserialize)]
struct SavedToken {
    access_token: twitch_oauth2::AccessToken,
    refresh_token: twitch_oauth2::RefreshToken,
}

// you should probably replace this with something more robust
#[cfg(debug_assertions)]
fn save_token(token: &twitch_oauth2::UserToken, save_path: &std::path::Path) -> Result<(), Error> {
    let token = SavedToken {
        access_token: token.access_token.clone(),
        refresh_token: token.refresh_token.clone().unwrap(),
    };
    let text = toml::to_string(&token)?;
    std::fs::write(save_path, text)?;
    Ok(())
}

#[cfg(debug_assertions)]
async fn load_token(
    path: &std::path::Path,
    client: &HelixClient<'_, reqwest::Client>,
) -> Result<Option<twitch_oauth2::UserToken>, Error> {
    let Some(text) = std::fs::read_to_string(path).ok() else {
        return Ok(None);
    };
    let token: SavedToken = toml::from_str(&text)?;
    Ok(Some(
        twitch_oauth2::UserToken::from_existing(
            client,
            token.access_token,
            token.refresh_token,
            None,
        )
        .await?,
    ))
}
