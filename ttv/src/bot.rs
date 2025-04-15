use std::{ops::Deref, sync::Arc};

use crossbeam_channel::Sender;
use tokio::sync::Mutex;
use twitch_api::{
    eventsub::{
        channel::{
            ChannelBanV1, ChannelChatClearV1, ChannelChatMessageDeleteV1, ChannelUnbanV1,
            ChannelWarningSendV1,
        },
        stream::{StreamOfflineV1, StreamOnlineV1},
        Event, Message, Transport,
    }, types::{Nickname, UserId}, HelixClient
};
use twitch_oauth2::{TwitchToken, UserToken};

use crate::{
    error::Error,
    queue::BotEvent,
    utils::save_token_to_db,
    websocket::{self, Broadcaster, SubscriptionIds},
};

use sea_orm::DbConn;

/// How often we should check if the token is still valid.
const TOKEN_VALIDATION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
/// The threshold at which we should refresh the token before expiration.
///
/// Only checked every [`TOKEN_VALIDATION_INTERVAL`] seconds.
const TOKEN_EXPIRATION_THRESHOLD: std::time::Duration = std::time::Duration::from_secs(60);

/// Represents the Twitch bot responsible for managing EventSub subscriptions and WebSocket connections.
pub struct TtvBot {
    /// The Helix client used for interacting with the Twitch API.
    pub(crate) client: HelixClient<'static, reqwest::Client>,
    /// A list of broadcaster IDs that the bot is monitoring.
    pub(crate) broadcasters: Vec<Arc<Mutex<UserToken>>>,
    /// The database connection used for storing and retrieving data.
    pub(crate) db: DbConn,
    /// The channel used for sending bot events to other parts of the application.
    pub(crate) sender: Sender<BotEvent>,
    /// The token of the bot used for auth.
    pub(crate) bot_token: Arc<Mutex<UserToken>>,
}

impl TtvBot {
    /// Starts the bot by initializing the WebSocket connection and token refresh loop.
    ///
    /// # Errors
    /// Returns an [`Error`] if the WebSocket connection or token refresh fails.
    #[fastrace::trace]
    pub async fn start(&self) -> Result<(), Error> {
        // Initialize the WebSocket client
        let websocket = websocket::TwitchWebSocketClient {
            session_id: None,
            client: self.client.clone(),
            connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            broadcaster_tokens: self.broadcasters.clone(),
            bot_token: self.bot_token.clone(),
        };

        // Define the bots own token in a loop
        let refresh_token = async move {
            let token = self.bot_token.clone();
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
    #[fastrace::trace]
    async fn handle_event(
        &self,
        event: Event,
        timestamp: twitch_api::types::Timestamp,
    ) -> Result<(), Error> {
        // Handle the event here
        match event {
            Event::ChannelChatClearV1(payload) => {
                if let Message::Notification(message) = payload.message {
                    self.sender
                        .send(BotEvent::ChatClear(message, timestamp))
                        .unwrap();
                }

                Ok(())
            }
            Event::ChannelChatMessageDeleteV1(payload) => {
                if let Message::Notification(message) = payload.message {
                    self.sender
                        .send(BotEvent::MessageDelete(message, timestamp))
                        .unwrap();
                }

                Ok(())
            }
            Event::ChannelBanV1(payload) => {
                if let Message::Notification(message) = payload.message {
                    self.sender.send(BotEvent::Ban(message, timestamp)).unwrap();
                }

                Ok(())
            }
            Event::ChannelUnbanV1(payload) => {
                if let Message::Notification(message) = payload.message {
                    self.sender
                        .send(BotEvent::Unban(message, timestamp))
                        .unwrap();
                }

                Ok(())
            }
            Event::ChannelWarningSendV1(payload) => {
                if let Message::Notification(message) = payload.message {
                    self.sender
                        .send(BotEvent::Warning(message, timestamp))
                        .unwrap();
                }

                Ok(())
            }
            Event::StreamOnlineV1(payload) => {
                if let Message::Notification(message) = payload.message {
                    self.sender
                        .send(BotEvent::StreamOnline(message, timestamp))
                        .unwrap();
                }

                Ok(())
            }
            Event::StreamOfflineV1(payload) => {
                if let Message::Notification(message) = payload.message {
                    self.sender
                        .send(BotEvent::StreamOffline(message, timestamp))
                        .unwrap();
                }

                Ok(())
            }
            _ => Ok(()),
        }
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
    #[fastrace::trace]
    async fn subscribe_events(
        &self,
        client: HelixClient<'static, reqwest::Client>,
        transport: Transport,
        token: &tokio::sync::MutexGuard<'_, UserToken>,
        ids: SubscriptionIds,
    ) -> Result<(), Error> {
        let token = token.deref();

        client
            .create_eventsub_subscription(
                StreamOfflineV1::broadcaster_user_id(ids.broadcaster_user_id()),
                transport.clone(),
                &*token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                StreamOnlineV1::broadcaster_user_id(ids.broadcaster_user_id()),
                transport.clone(),
                &*token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelBanV1::broadcaster_user_id(ids.broadcaster_user_id()),
                transport.clone(),
                &*token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelUnbanV1::broadcaster_user_id(ids.broadcaster_user_id()),
                transport.clone(),
                &*token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelChatMessageDeleteV1::new(ids.broadcaster_user_id(), ids.user_id()),
                transport.clone(),
                &*token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelChatClearV1::new(ids.broadcaster_user_id(), ids.user_id()),
                transport.clone(),
                &*token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelWarningSendV1::new(ids.broadcaster_user_id(), ids.user_id()),
                transport.clone(),
                &*token,
            )
            .await?;

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
#[fastrace::trace]
async fn refresh_and_validate_token(
    db: &DbConn,
    token: &mut UserToken,
    client: &HelixClient<'_, reqwest::Client>,
) -> Result<(), Error> {
    // Check if the token is close to expiration
    if token.expires_in() < TOKEN_EXPIRATION_THRESHOLD {
        log::info!("refreshed token");

        // Refresh the token
        token.refresh_token(client).await?;
        // Save the refreshed token to the database
        save_token_to_db(db, token).await?;
    }

    // Validate the token
    token.validate_token(client).await?;

    Ok(())
}
