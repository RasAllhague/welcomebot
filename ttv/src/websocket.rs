use std::sync::Arc;

use futures::TryStreamExt;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite;
use tracing::Instrument;
use twitch_api::{
    eventsub::{
        self,
        Event, EventsubWebsocketData, ReconnectPayload, SessionData, Transport, WelcomePayload,
    },
    types::{self, UserId},
    HelixClient,
};
use twitch_oauth2::{TwitchToken, UserToken};

use crate::error::Error;

/// A client for connecting to the Twitch websocket
pub struct TwitchWebsocketClient {
    /// The session id of the websocket connection
    pub session_id: Option<String>,
    /// The token used to authenticate with the Twitch API
    pub token: Arc<Mutex<UserToken>>,
    /// The client used to make requests to the Twitch API
    pub client: HelixClient<'static, reqwest::Client>,
    /// The url to use for websocket
    pub connect_url: url::Url,
    /// Chats to connect to.
    pub chats: Vec<twitch_api::types::UserId>,
}

impl TwitchWebsocketClient {
    /// Connect to the websocket and return the stream
    async fn connect(
        &self,
    ) -> Result<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Error,
    > {
        tracing::info!("connecting to twitch");

        let config = tungstenite::protocol::WebSocketConfig::default();
        let (socket, _) =
            tokio_tungstenite::connect_async_with_config(&self.connect_url, Some(config), false)
                .await?;

        Ok(socket)
    }

    /// Run the websocket subscriber
    #[tracing::instrument(name = "subscriber", skip_all, fields())]
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
        // Establish the stream
        let mut s = self.connect().await?;

        // Loop over the stream, processing messages as they come in.
        while let Some(msg) = futures::StreamExt::next(&mut s).await {
            let span = tracing::debug_span!("message received", raw_message = ?msg);

            let msg = match msg {
                Err(tungstenite::Error::Protocol(
                    tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
                )) => {
                    tracing::warn!(
                        "connection was sent an unexpected frame or was reset, reestablishing it"
                    );

                    s = self.connect().instrument(span).await?;
                    continue;
                }
                _ => msg?,
            };

            self.process_message(msg, &mut event_fn, &mut subscribe_fn)
                .instrument(span)
                .await?
        }

        Ok(())
    }

    /// Process a message from the websocket
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
                tracing::trace!("{s}");

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
        tracing::info!("connected to twitch chat");

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

#[derive(Debug, Clone)]
pub struct SubscriptionIds {
    broadcaster_user_id: UserId,
    user_id: UserId,
}

impl SubscriptionIds {
    pub fn new(broadcaster_user_id: UserId, user_id: UserId) -> Self {
        Self {
            broadcaster_user_id,
            user_id,
        }
    }

    pub fn broadcaster_user_id(&self) -> UserId {
        self.broadcaster_user_id.clone()
    }

    pub fn user_id(&self) -> UserId {
        self.user_id.clone()
    }
}
