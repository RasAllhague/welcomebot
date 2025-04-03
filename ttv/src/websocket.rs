use std::sync::Arc;

use futures::TryStreamExt;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite;
use twitch_api::{
    eventsub::{
        self, Event, EventsubWebsocketData, ReconnectPayload, SessionData, Transport,
        WelcomePayload,
    },
    types::{self, UserId},
    HelixClient,
};
use twitch_oauth2::{TwitchToken, UserToken};

use crate::error::Error;

/// A client for connecting to the Twitch WebSocket.
pub struct TwitchWebSocketClient {
    /// The session ID of the WebSocket connection.
    pub session_id: Option<String>,
    /// The token used to authenticate with the Twitch API.
    pub token: Arc<Mutex<UserToken>>,
    /// The client used to make requests to the Twitch API.
    pub client: HelixClient<'static, reqwest::Client>,
    /// The URL to use for the WebSocket connection.
    pub connect_url: url::Url,
    /// A list of chats to connect to.
    pub chats: Vec<twitch_api::types::UserId>,
}

impl TwitchWebSocketClient {
    /// Connects to the Twitch WebSocket and returns the WebSocket stream.
    ///
    /// # Errors
    /// Returns an [`Error`] if the connection to the WebSocket fails.
    #[fastrace::trace]
    async fn connect(
        &self,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Error,
    > {
        log::info!("Connecting to Twitch WebSocket");

        let config = tungstenite::protocol::WebSocketConfig::default();
        let (socket, _) =
            tokio_tungstenite::connect_async_with_config(&self.connect_url, Some(config), false)
                .await?;

        Ok(socket)
    }

    /// Runs the WebSocket subscriber, processing incoming messages and managing subscriptions.
    ///
    /// # Arguments
    /// * `event_fn` - A function to handle incoming EventSub events.
    /// * `subscribe_fn` - A function to handle subscription management.
    ///
    /// # Errors
    /// Returns an [`Error`] if processing messages or managing subscriptions fails.
    #[fastrace::trace]
    pub async fn run<Fut, Fut2>(
        mut self,
        mut event_fn: impl FnMut(Event, types::Timestamp) -> Fut,
        mut subscribe_fn: impl FnMut(
            HelixClient<'static, reqwest::Client>,
            Transport,
            Arc<Mutex<UserToken>>,
            SubscriptionIds,
        ) -> Fut2,
    ) -> Result<(), Error>
    where
        Fut: std::future::Future<Output = Result<(), Error>>,
        Fut2: std::future::Future<Output = Result<(), Error>>,
    {
        // Establish the WebSocket stream
        let mut s = self.connect().await?;

        // Loop over the stream, processing messages as they come in
        while let Some(msg) = futures::StreamExt::next(&mut s).await {
            log::info!("message received {:?}", msg);

            let msg = match msg {
                Err(tungstenite::Error::Protocol(
                    tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                )) => {
                    log::warn!(
                        "Connection was reset or sent an unexpected frame, reestablishing it"
                    );

                    s = self.connect().await?;
                    continue;
                }
                _ => msg?,
            };

            self.process_message(msg, &mut event_fn, &mut subscribe_fn)
                .await?
        }

        Ok(())
    }

    /// Processes a single message from the WebSocket.
    ///
    /// # Arguments
    /// * `msg` - The WebSocket message to process.
    /// * `event_fn` - A function to handle incoming EventSub events.
    /// * `subscribe_fn` - A function to handle subscription management.
    ///
    /// # Errors
    /// Returns an [`Error`] if processing the message fails.
    #[fastrace::trace]
    async fn process_message<Fut, Fut2>(
        &mut self,
        msg: tungstenite::Message,
        event_fn: &mut impl FnMut(Event, types::Timestamp) -> Fut,
        subscribe_fn: &mut impl FnMut(
            HelixClient<'static, reqwest::Client>,
            Transport,
            Arc<Mutex<UserToken>>,
            SubscriptionIds,
        ) -> Fut2,
    ) -> Result<(), Error>
    where
        Fut: std::future::Future<Output = Result<(), Error>>,
        Fut2: std::future::Future<Output = Result<(), Error>>,
    {
        match msg {
            tungstenite::Message::Text(s) => {
                log::trace!("{s}");

                // Parse the message into a [twitch_api::eventsub::EventsubWebsocketData]
                match Event::parse_websocket(&s)? {
                    EventsubWebsocketData::Welcome {
                        payload: WelcomePayload { session },
                        ..
                    }
                    | EventsubWebsocketData::Reconnect {
                        payload: ReconnectPayload { session },
                        ..
                    } => {
                        self.process_welcome_message(session, subscribe_fn).await?;
                        Ok(())
                    }
                    EventsubWebsocketData::Notification { metadata, payload } => {
                        event_fn(payload, metadata.message_timestamp.into_owned()).await?;
                        Ok(())
                    }
                    EventsubWebsocketData::Revocation { metadata, payload } => {
                        Err(Error::TokenRevoked(format!(
                            "metadata: {:?}, payload: {:?}",
                            metadata, payload
                        )))
                    }
                    EventsubWebsocketData::Keepalive {
                        metadata: _,
                        payload: _,
                    } => Ok(()),
                    _ => Ok(()),
                }
            }
            tungstenite::Message::Close(_) => todo!(),
            _ => Ok(()),
        }
    }

    /// Processes the welcome message from the WebSocket and manages subscriptions.
    ///
    /// # Arguments
    /// * `data` - The session data from the welcome message.
    /// * `subscribe_fn` - A function to handle subscription management.
    ///
    /// # Errors
    /// Returns an [`Error`] if processing the welcome message or managing subscriptions fails.
    #[fastrace::trace]
    async fn process_welcome_message<Fut>(
        &mut self,
        data: SessionData<'_>,
        subscribe_fn: &mut impl FnMut(
            HelixClient<'static, reqwest::Client>,
            Transport,
            Arc<Mutex<UserToken>>,
            SubscriptionIds,
        ) -> Fut,
    ) -> Result<(), Error>
    where
        Fut: std::future::Future<Output = Result<(), Error>>,
    {
        log::info!("Connected to Twitch WebSocket");

        self.session_id = Some(data.id.to_string());

        if let Some(url) = data.reconnect_url {
            self.connect_url = url.parse()?;
        }

        let token = self.token.lock().await;
        let transport = eventsub::Transport::websocket(data.id.clone());

        for id in &self.chats {
            let user_id = token.user_id().unwrap().to_owned();
            let subs: Vec<_> = self
                .client
                .get_eventsub_subscriptions(Some(eventsub::Status::Enabled), None, None, &*token)
                .map_ok(|r| {
                    futures::stream::iter(
                        r.subscriptions
                            .into_iter()
                            .filter(|s| {
                                s.transport
                                    .as_websocket()
                                    .is_some_and(|t| t.session_id == data.id)
                            })
                            .map(Ok::<_, Error>),
                    )
                })
                .try_flatten()
                .try_collect()
                .await?;

            if !subs.is_empty() {
                continue;
            }

            subscribe_fn(
                self.client.clone(),
                transport.clone(),
                self.token.clone(),
                SubscriptionIds::new(id.clone(), user_id.clone()),
            )
            .await?;
        }

        Ok(())
    }
}

/// Represents the IDs required for EventSub subscriptions.
#[derive(Debug, Clone)]
pub struct SubscriptionIds {
    /// The broadcaster's user ID.
    broadcaster_user_id: UserId,
    /// The user's ID.
    user_id: UserId,
}

impl SubscriptionIds {
    /// Creates a new `SubscriptionIds` instance.
    ///
    /// # Arguments
    /// * `broadcaster_user_id` - The broadcaster's user ID.
    /// * `user_id` - The user's ID.
    pub fn new(broadcaster_user_id: UserId, user_id: UserId) -> Self {
        Self {
            broadcaster_user_id,
            user_id,
        }
    }

    /// Returns the broadcaster's user ID.
    pub fn broadcaster_user_id(&self) -> UserId {
        self.broadcaster_user_id.clone()
    }

    /// Returns the user's ID.
    pub fn user_id(&self) -> UserId {
        self.user_id.clone()
    }
}
