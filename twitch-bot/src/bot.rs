use ttv::{
    bot::TwitchBot,
    error::Error,
    websocket::{SubscriptionIds, TwitchClient, UserTokenArc},
};
use twitch_api::eventsub::{
    Event, Message, Transport,
    channel::{
        ChannelBanV1, ChannelChatClearV1, ChannelChatMessageDeleteV1, ChannelUnbanV1,
        ChannelWarningSendV1,
    },
    stream::{StreamOfflineV1, StreamOnlineV1},
};

use sea_orm::DbConn;
use twitch_oauth2::TwitchToken;

use crate::utils::save_token_to_db;

/// The threshold at which we should refresh the token before expiration.
///
/// Only checked every [`TOKEN_VALIDATION_INTERVAL`] seconds.
const TOKEN_EXPIRATION_THRESHOLD: std::time::Duration = std::time::Duration::from_secs(60);

/// Represents the Twitch bot responsible for managing EventSub subscriptions and WebSocket connections.
pub struct TtvBot {
    /// The Helix client used for interacting with the Twitch API.
    pub(crate) client: TwitchClient,
    /// A list of broadcaster IDs that the bot is monitoring.
    pub(crate) broadcasters: Vec<UserTokenArc>,
    /// The database connection used for storing and retrieving data.
    pub(crate) db: DbConn,
    /// The token of the bot used for auth.
    pub(crate) bot_token: UserTokenArc,
}

impl TwitchBot for TtvBot {
    fn client(&self) -> &TwitchClient {
        &self.client
    }

    fn broadcaster_tokens(&self) -> &[UserTokenArc] {
        &self.broadcasters
    }

    fn bot_token(&self) -> UserTokenArc {
        self.bot_token.clone()
    }

    #[fastrace::trace]
    async fn refresh_token(
        &self,
        user_token: UserTokenArc,
        client: &TwitchClient,
    ) -> Result<(), Error> {
        let mut token = user_token.lock().await;

        // Check if the token is close to expiration
        if token.expires_in() < TOKEN_EXPIRATION_THRESHOLD {
            log::info!("refreshed token");

            // Refresh the token
            token.refresh_token(client).await?;
            // Save the refreshed token to the database
            save_token_to_db(&self.db, &token)
                .await
                .map_err(|err| ttv::error::Error::CustomError(Box::new(err)))?;
        }

        Ok(())
    }

    #[fastrace::trace]
    async fn handle_event(
        &self,
        event: Event,
        _timestamp: twitch_api::types::Timestamp,
    ) -> Result<(), Error> {
        // Handle the event here
        match event {
            Event::ChannelChatClearV1(payload) => {
                if let Message::Notification(_message) = payload.message {
                    println!("ChannelChatClearV1 received!");
                }

                Ok(())
            }
            Event::ChannelChatMessageDeleteV1(payload) => {
                if let Message::Notification(_message) = payload.message {
                    println!("ChannelChatMessageDeleteV1 received!");
                }

                Ok(())
            }
            Event::ChannelBanV1(payload) => {
                if let Message::Notification(_message) = payload.message {
                    println!("ChannelBanV1 received!");
                }

                Ok(())
            }
            Event::ChannelUnbanV1(payload) => {
                if let Message::Notification(_message) = payload.message {
                    println!("ChannelUnbanV1 received!");
                }

                Ok(())
            }
            Event::ChannelWarningSendV1(payload) => {
                if let Message::Notification(_message) = payload.message {
                    println!("ChannelWarningSendV1 received!");
                }

                Ok(())
            }
            Event::StreamOnlineV1(payload) => {
                if let Message::Notification(_message) = payload.message {
                    println!("StreamOnlineV1 received!");
                }

                Ok(())
            }
            Event::StreamOfflineV1(payload) => {
                if let Message::Notification(_message) = payload.message {
                    println!("StreamOfflineV1 received!");
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    #[fastrace::trace]
    async fn subscribe_events(
        &self,
        client: TwitchClient,
        transport: Transport,
        token: twitch_oauth2::UserToken,
        ids: SubscriptionIds,
    ) -> Result<(), Error> {
        client
            .create_eventsub_subscription(
                StreamOfflineV1::broadcaster_user_id(ids.broadcaster_user_id()),
                transport.clone(),
                &token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                StreamOnlineV1::broadcaster_user_id(ids.broadcaster_user_id()),
                transport.clone(),
                &token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelBanV1::broadcaster_user_id(ids.broadcaster_user_id()),
                transport.clone(),
                &token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelUnbanV1::broadcaster_user_id(ids.broadcaster_user_id()),
                transport.clone(),
                &token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelChatMessageDeleteV1::new(ids.broadcaster_user_id(), ids.user_id()),
                transport.clone(),
                &token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelChatClearV1::new(ids.broadcaster_user_id(), ids.user_id()),
                transport.clone(),
                &token,
            )
            .await?;
        client
            .create_eventsub_subscription(
                ChannelWarningSendV1::new(ids.broadcaster_user_id(), ids.user_id()),
                transport.clone(),
                &token,
            )
            .await?;

        Ok(())
    }
}
