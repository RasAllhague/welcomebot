use sea_orm::DbErr;
use thiserror::Error;
use twitch_api::client::CompatError;
use twitch_oauth2::tokens::errors::ValidationError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database operation failed: {0}")]
    DbErr(#[from] DbErr),

    #[error("Twitch token validation failed: {0}")]
    TokenValidationError(#[from] ValidationError<CompatError<reqwest::Error>>),

    #[error("Twitch event sub failed: {0}")]
    TtvError(#[from] ttv::error::Error),
}
