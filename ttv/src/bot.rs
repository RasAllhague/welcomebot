use std::sync::Arc;

use tokio::sync::Mutex;
use twitch_api::{
    eventsub::{Event, Transport},
    HelixClient,
};
use twitch_oauth2::{Scope, TwitchToken, UserToken};

use crate::{
    error::Error,
    utils::save_token_to_db,
    websocket::{self, SubscriptionIds},
};

use sea_orm::DbConn;

/// How often we should check if the token is still valid.
const TOKEN_VALIDATION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
/// The threshold at which we should refresh the token before expiration.
///
/// Only checked every [`TOKEN_VALIDATION_INTERVAL`] seconds.
const TOKEN_EXPIRATION_THRESHOLD: std::time::Duration = std::time::Duration::from_secs(60);
/// The scopes we need for the bot.
pub const SCOPES: &[Scope] = &[Scope::UserReadChat, Scope::UserWriteChat];

/// Represents the Twitch bot responsible for managing EventSub subscriptions and WebSocket connections.
pub struct TtvBot {
    /// The Helix client used for interacting with the Twitch API.
    pub(crate) client: HelixClient<'static, reqwest::Client>,
    /// The user token used for authentication.
    pub(crate) token: Arc<Mutex<twitch_oauth2::UserToken>>,
    /// A list of broadcaster IDs that the bot is monitoring.
    pub(crate) broadcasters: Vec<twitch_api::types::UserId>,
    /// The database connection used for storing and retrieving data.
    pub(crate) db: DbConn,
}

impl TtvBot {
    /// Starts the bot by initializing the WebSocket connection and token refresh loop.
    ///
    /// # Errors
    /// Returns an [`Error`] if the WebSocket connection or token refresh fails.
    pub async fn start(&self) -> Result<(), Error> {
        // Initialize the WebSocket client
        let websocket = websocket::TwitchWebSocketClient {
            session_id: None,
            token: self.token.clone(),
            client: self.client.clone(),
            connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            chats: self.broadcasters.clone(),
        };

        // Define the token refresh loop
        let refresh_token = async move {
            let token = self.token.clone();
            let client = self.client.clone();

            let mut interval = tokio::time::interval(TOKEN_VALIDATION_INTERVAL);

            loop {
                interval.tick().await;
                let mut token = token.lock().await;
                refresh_and_validate_token(&self.db, &mut token, &client).await?;
            }

            #[allow(unreachable_code)]
            Ok(())
        };

        // Run the WebSocket client and token refresh loop concurrently
        let ws = websocket.run(
            |e, ts| async { self.handle_event(e, ts).await },
            |client, transport, token, ids| async {
                self.subscribe_events(client, transport, token, ids).await
            },
        );

        futures::future::try_join(ws, refresh_token).await?;

        Ok(())
    }

    /// Handles all EventSub events received from the WebSocket connection.
    ///
    /// # Arguments
    /// * `event` - The EventSub event received.
    /// * `timestamp` - The timestamp of the event.
    ///
    /// # Errors
    /// Returns an [`Error`] if handling the event fails.
    async fn handle_event(
        &self,
        event: Event,
        timestamp: twitch_api::types::Timestamp,
    ) -> Result<(), Error> {
        // Handle the event here
        Ok(())
    }

    /// Subscribes to EventSub events for the specified broadcaster.
    ///
    /// # Arguments
    /// * `client` - The Helix client used for interacting with the Twitch API.
    /// * `transport` - The transport method for the subscription (e.g., WebSocket).
    /// * `token` - The user token used for authentication.
    /// * `ids` - The subscription IDs for the events.
    ///
    /// # Errors
    /// Returns an [`Error`] if subscribing to the events fails.
    async fn subscribe_events(
        &self,
        client: HelixClient<'static, reqwest::Client>,
        transport: Transport,
        token: Arc<Mutex<UserToken>>,
        ids: SubscriptionIds,
    ) -> Result<(), Error> {
        // Subscribe to events here
        Ok(())
    }
}

/// Refreshes and validates the user token.
///
/// # Arguments
/// * `db` - The database connection used for storing the token.
/// * `token` - The user token to refresh and validate.
/// * `client` - The Helix client used for interacting with the Twitch API.
///
/// # Errors
/// Returns an [`Error`] if refreshing or validating the token fails.
async fn refresh_and_validate_token(
    db: &DbConn,
    token: &mut UserToken,
    client: &HelixClient<'_, reqwest::Client>,
) -> Result<(), Error> {
    // Check if the token is close to expiration
    if token.expires_in() < TOKEN_EXPIRATION_THRESHOLD {
        tracing::info!("refreshed token");

        // Refresh the token
        token.refresh_token(client).await?;
        // Save the refreshed token to the database
        save_token_to_db(db, token).await?;
    }

    // Validate the token
    token.validate_token(client).await?;

    Ok(())
}
