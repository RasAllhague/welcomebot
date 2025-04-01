use sea_orm::DbErr;
use thiserror::Error;
use twitch_api::{client::CompatError, eventsub};
use twitch_oauth2::tokens::errors::{RefreshTokenError, ValidationError};

/// Represents errors that can occur in the application.
#[derive(Error, Debug)]
pub enum Error {
    /// Error that occurs during a WebSocket operation using Tungstenite.
    ///
    /// This error is typically caused by issues with the WebSocket connection.
    #[error("Failed to connect to do websocket operation")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    /// Error indicating that the token has been revoked.
    ///
    /// # Fields
    /// - `String`: A message or identifier related to the revoked token.
    #[error("The token has been revoked")]
    TokenRevoked(String),

    /// Error that occurs when a Twitch API request fails.
    ///
    /// This error is typically caused by issues with the HTTP client or the Twitch API.
    #[error("Failed to send twitch request")]
    ClientRequestError(#[from] twitch_api::helix::ClientRequestError<reqwest::Error>),

    /// Error that occurs when a URL cannot be parsed.
    ///
    /// This error is typically caused by invalid or malformed URLs.
    #[error("Failed to parse url")]
    UrlParseError(#[from] url::ParseError),

    /// Error that occurs when a payload cannot be parsed.
    ///
    /// This error is typically caused by invalid or unexpected payload data.
    #[error("Failed to parse payload")]
    PayloadParseError(#[from] eventsub::PayloadParseError),

    /// Error that occurs when a payload cannot be serialized.
    #[error("Failed to serialize toml")]
    TomlDeserializeError(#[from] toml::de::Error),

    /// Error that occurs when a payload cannot be serialized.
    #[error("Failed to serialize toml")]
    TomlSerializeError(#[from] toml::ser::Error),

    #[error("Failed to validate twitch token")]
    TokenValidationError(#[from] ValidationError<CompatError<reqwest::Error>>),

    #[error("Failed to refresh twitch token")]
    RefreshTokenError(#[from] RefreshTokenError<CompatError<reqwest::Error>>),

    #[error("Failed io operation on file")]
    Io(#[from] std::io::Error),

    #[error("Failed to do database operation")]
    DbError(#[from] DbErr) 
}
