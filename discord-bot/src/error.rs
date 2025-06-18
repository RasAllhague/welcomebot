use poise::serenity_prelude;
use thiserror::Error;

/// Represents errors that can occur in the application.
///
/// This enum consolidates various error types that may arise during the
/// execution of the bot, such as invalid channels, Serenity errors, or
/// errors from external modules like image generation or Twitch client.
#[derive(Error, Debug)]
pub enum Error {
    /// Error indicating that the specified channel is not a valid text channel.
    ///
    /// # Fields
    /// - `String`: The name or ID of the invalid channel.
    #[error("The channel `{0}` is not a valid channel.")]
    InvalidTextChannel(String),

    /// Error that occurs when an error is encountered in the Serenity library.
    ///
    /// This error is typically caused by issues with Discord API interactions.
    #[error("An error in serenity occurred.")]
    Serenity(#[from] serenity_prelude::Error),

    /// Error that occurs during image generation.
    ///
    /// This error is typically caused by issues in the `img_gen` module.
    #[error("An error occurred during image generation.")]
    ImageGen(#[from] img_gen::error::Error),
}
