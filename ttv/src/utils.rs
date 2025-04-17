use entity::twitch_broadcaster::Model;
use sea_orm::{sqlx::types::chrono::Utc, DbConn};
use twitch_oauth2::{AccessToken, RefreshToken, UserToken};
use welcome_service::{twitch_broadcaster_mutation, twitch_broadcaster_query};

use crate::{error::Error, websocket::TwitchClient};

/// Saves the provided user token to the database.
///
/// # Arguments
/// * `db` - The database connection used to store the token.
/// * `token` - The user token to save.
///
/// # Errors
/// Returns an [`Error`] if saving the token to the database fails.
pub async fn save_token_to_db(db: &DbConn, token: &UserToken) -> Result<(), Error> {
    if let Some(mut model) =
        twitch_broadcaster_query::get_by_broadcaster_id(db, token.user_id.as_str()).await?
    {
        model.access_token = token.access_token.secret().to_string();
        model.refresh_token = token.refresh_token.as_ref().map(|x| x.secret().to_string());
        model.broadcaster_login = token.login.to_string();
        model.broadcaster_name = token.login.to_string();

        twitch_broadcaster_mutation::update(db, model).await?;
    } else {
        // Create a database model for the token
        let token = Model {
            id: 0,
            broadcaster_login: token.login.to_string(),
            broadcaster_id: token.user_id.to_string(),
            broadcaster_name: token.login.to_string(),
            access_token: token.access_token.secret().to_string(),
            refresh_token: token.refresh_token.as_ref().map(|x| x.secret().to_string()),
            last_refreshed: None,
            create_date: Utc::now(),
            modify_date: None,
        };

        // Save or update the token in the database
        twitch_broadcaster_mutation::create(db, token).await?;
    }
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
    client: &TwitchClient,
    broadcaster_login: &str,
) -> Result<Option<UserToken>, Error> {
    if let Some(token_model) =
        twitch_broadcaster_query::get_by_broadcaster_login(db, broadcaster_login).await?
    {
        create_user_token_from_model(client, &token_model)
            .await
            .map(|token| Some(token))
    } else {
        Ok(None)
    }
}

async fn create_user_token_from_model(
    client: &TwitchClient,
    token_model: &Model,
) -> Result<UserToken, Error> {
    let access_token = AccessToken::new(token_model.access_token.clone());

    // Extract the refresh token, if it exists
    let refresh_token = token_model
        .refresh_token
        .clone()
        .map(|x| RefreshToken::new(x));

    // Create a UserToken from the retrieved data
    let token =
        UserToken::from_existing(client, access_token, refresh_token, None).await?;

    Ok(token)
}
