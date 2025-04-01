use poise::serenity_prelude;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The channel `{0}` is not a valid channel.")]
    InvalidTextChannel(String),

    #[error("An error in serenity occured.")]
    Serenity(#[from] serenity_prelude::Error),

    #[error("An error occured during image generation.")]
    ImageGen(#[from] img_gen::error::Error),
}
