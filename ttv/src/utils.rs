use entity::twitch_token::Model;
use sea_orm::{sqlx::types::chrono::Utc, DbConn};
use twitch_api::HelixClient;
use twitch_oauth2::{AccessToken, RefreshToken};
use welcome_service::{twitch_token_mutation, twitch_token_query};

use crate::error::Error;

/// Saves the provided user token to the database.
///
/// # Arguments
/// * `db` - The database connection used to store the token.
/// * `token` - The user token to save.
///
/// # Errors
/// Returns an [`Error`] if saving the token to the database fails.
pub async fn save_token_to_db(db: &DbConn, token: &twitch_oauth2::UserToken) -> Result<(), Error> {
    // Create a database model for the token
    let token = Model {
        id: 0,
        access_token: Some(token.access_token.to_string()),
        refresh_token: token.refresh_token.clone().map(|x| x.to_string()),
        last_refreshed: Some(Utc::now()),
    };

    // Save or update the token in the database
    twitch_token_mutation::create_or_update(db, token).await?;

    Ok(())
}

/// Loads a user token from the database and validates it.
///
/// # Arguments
/// * `db` - The database connection used to retrieve the token.
/// * `client` - The Helix client used to validate the token.
///
/// # Returns
/// Returns an [`Option`] containing the user token if it exists and is valid, or `None` if no valid token is found.
///
/// # Errors
/// Returns an [`Error`] if retrieving or validating the token fails.
#[fastrace::trace]
pub async fn load_token_from_db(
    db: &DbConn,
    client: &HelixClient<'_, reqwest::Client>,
) -> Result<Option<twitch_oauth2::UserToken>, Error> {
    // Attempt to retrieve the token from the database
    match twitch_token_query::get(db).await? {
        Some(token) => {
            // Extract the access token, returning None if it doesn't exist
            let Some(access_token) = token.access_token.map(|x| AccessToken::new(x)) else {
                return Ok(None);
            };

            // Extract the refresh token, if it exists
            let refresh_token = token.refresh_token.map(|x| RefreshToken::new(x));

            // Create a UserToken from the retrieved data
            let token =
                twitch_oauth2::UserToken::from_existing(client, access_token, refresh_token, None)
                    .await?;

            Ok(Some(token))
        }
        // Return None if no token is found in the database
        _ => Ok(None),
    }
}
