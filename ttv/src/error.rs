use thiserror::Error;
use twitch_api::{client::CompatError, eventsub, types::UserName};
use twitch_oauth2::tokens::errors::{
    DeviceUserTokenExchangeError, RefreshTokenError, UserTokenExchangeError, ValidationError,
};

/// Represents errors that can occur in the Twitch bot application.
///
/// This enum consolidates various error types that may arise during the
/// execution of the Twitch bot, such as WebSocket issues, token validation
/// failures, database errors, or Twitch API request failures.
#[derive(Error, Debug)]
pub enum Error {
    /// Error that occurs during a WebSocket operation using Tungstenite.
    ///
    /// This error is typically caused by issues with the WebSocket connection.
    #[error("WebSocket operation failed: {0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    /// Error indicating that the token has been revoked.
    ///
    /// # Fields
    /// - `String`: A message or identifier related to the revoked token.
    #[error("The token has been revoked: {0}")]
    TokenRevoked(String),

    /// Error that occurs when a Twitch API request fails.
    ///
    /// This error is typically caused by issues with the HTTP client or the Twitch API.
    #[error("Twitch API request failed: {0}")]
    ClientRequestError(#[from] twitch_api::helix::ClientRequestError<reqwest::Error>),

    /// Error that occurs when a URL cannot be parsed.
    ///
    /// This error is typically caused by invalid or malformed URLs.
    #[error("Failed to parse URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// Error that occurs when a payload cannot be parsed.
    ///
    /// This error is typically caused by invalid or unexpected payload data.
    #[error("Failed to parse payload: {0}")]
    PayloadParseError(#[from] eventsub::PayloadParseError),

    /// Error that occurs when a payload cannot be deserialized from TOML format.
    ///
    /// This error is typically caused by invalid or malformed TOML data.
    #[error("Failed to deserialize TOML data: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),

    /// Error that occurs when a payload cannot be serialized to TOML format.
    ///
    /// This error is typically caused by issues with the serialization process.
    #[error("Failed to serialize TOML data: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),

    /// Error that occurs when validating a Twitch token fails.
    ///
    /// This error is typically caused by invalid or expired tokens.
    #[error("Twitch token validation failed: {0}")]
    TokenValidationError(#[from] ValidationError<CompatError<reqwest::Error>>),

    /// Error that occurs when refreshing a Twitch token fails.
    ///
    /// This error is typically caused by issues with the token refresh process.
    #[error("Failed to refresh Twitch token: {0}")]
    RefreshTokenError(#[from] RefreshTokenError<CompatError<reqwest::Error>>),

    /// Error that occurs during an I/O operation.
    ///
    /// This error is typically caused by issues with file or network operations.
    #[error("I/O operation failed: {0}")]
    Io(#[from] std::io::Error),

    /// Error that occurs when exchanging a device token fails.
    ///
    /// This error is typically caused by issues during the device token exchange process.
    #[error("Device token exchange failed: {0}")]
    DeviceUserTokenExchangeError(#[from] DeviceUserTokenExchangeError<CompatError<reqwest::Error>>),

    #[error("User token exchange failed: {0}")]
    UserTokenExchangeError(#[from] UserTokenExchangeError<CompatError<reqwest::Error>>),

    /// Error indicating that a broadcaster was not found.
    ///
    /// This error occurs when the specified broadcaster login does not exist or cannot be retrieved.
    #[error("Broadcaster not found: {0}")]
    BroadcasterNotFound(String),

    #[error("The provided oauth url is not valid: {0}")]
    InvalidOAuthUrl(String),

    #[error("twitch oauth errored with error: {error} - {error_description}")]
    TwitchOAuthFailure {
        error: String,
        error_description: String,
    },

    #[error("Bot username not found: {0}")]
    BotTokenNotFound(UserName),

    #[error("Custom error received: {0}")]
    CustomError(Box<dyn std::error::Error>)
}
