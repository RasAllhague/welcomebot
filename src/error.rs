use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The channel `{0}` is not a valid channel.")]
    InvalidTextChannel(String),
}