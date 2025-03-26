use thiserror::Error;
use twitch_api::eventsub;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to connect to do websocket operation")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("The token has been revoked")]
    TokenRevoked(String),
    #[error("Failed to send twitch request")]
    ClientRequestError(#[from] twitch_api::helix::ClientRequestError<reqwest::Error>),
    #[error("Failed to parse url")]
    UrlParseError(#[from] url::ParseError),
    #[error("Failed to parse payload")]
    PayloadParseError(#[from] eventsub::PayloadParseError),
}
