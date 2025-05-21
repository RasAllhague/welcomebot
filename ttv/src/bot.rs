use twitch_api::eventsub::{Event, Transport};
use twitch_oauth2::{TwitchToken, UserToken};

use crate::{
    error::Error,
    websocket::{self, SubscriptionIds, TwitchClient, UserTokenArc},
};

/// How often we should check if the token is still valid.
const TOKEN_VALIDATION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);

pub trait TwitchBot {
    fn client(&self) -> &TwitchClient;
    fn broadcaster_tokens(&self) -> &[UserTokenArc];
    fn bot_token(&self) -> UserTokenArc;

    #[allow(async_fn_in_trait)]
    #[fastrace::trace]
    async fn start(&self) -> Result<(), Error> {
        // Initialize the WebSocket client
        let websocket = websocket::TwitchWebSocketClient {
            session_id: None,
            client: self.client().clone(),
            connect_url: twitch_api::TWITCH_EVENTSUB_WEBSOCKET_URL.clone(),
            broadcaster_tokens: self.broadcaster_tokens().to_vec(),
            bot_token: self.bot_token().clone(),
        };

        // Define the bots own token in a loop
        let refresh_bot_token = async move {
            let token = self.bot_token().clone();
            let client = self.client().clone();

            let mut interval = tokio::time::interval(TOKEN_VALIDATION_INTERVAL);

            loop {
                interval.tick().await;

                self.refresh_token(token.clone(), &client).await?;
                token.lock().await.validate_token(self.client()).await?;
            }

            #[allow(unreachable_code)]
            Ok(())
        };

        let refresh_broadcaster_tokens = async move {
            let broadcaster_tokens = self.broadcaster_tokens();
            let client = self.client().clone();

            let mut interval = tokio::time::interval(TOKEN_VALIDATION_INTERVAL);

            loop {
                interval.tick().await;

                for broadcaster_token in broadcaster_tokens {
                    self.refresh_token(broadcaster_token.clone(), &client)
                        .await?;
                    broadcaster_token
                        .lock()
                        .await
                        .validate_token(self.client())
                        .await?;
                }
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

        futures::future::try_join3(ws, refresh_bot_token, refresh_broadcaster_tokens).await?;

        Ok(())
    }

    fn refresh_token(
        &self,
        user_token: UserTokenArc,
        client: &TwitchClient,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    fn handle_event(
        &self,
        event: Event,
        timestamp: twitch_api::types::Timestamp,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;

    fn subscribe_events(
        &self,
        client: TwitchClient,
        transport: Transport,
        token: UserToken,
        ids: SubscriptionIds,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
