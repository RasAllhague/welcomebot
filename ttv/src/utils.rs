use entity::twitch_token::Model;
use sea_orm::{sqlx::types::chrono::Utc, DbConn};
use twitch_api::HelixClient;
use twitch_oauth2::{AccessToken, RefreshToken};
use welcome_service::{twitch_token_mutation, twitch_token_query};

use crate::error::Error;

pub async fn save_token_to_db(db: &DbConn, token: &twitch_oauth2::UserToken) -> Result<(), Error> {
    let token = Model {
        id: 0,
        access_token: Some(token.access_token.to_string()),
        refresh_token: token.refresh_token.clone().map(|x| x.to_string()),
        last_refreshed: Some(Utc::now()),
    };

    twitch_token_mutation::create_or_update(db, token).await?;

    Ok(())
}

pub async fn load_token_from_db(
    db: &DbConn,
    client: &HelixClient<'_, reqwest::Client>,
) -> Result<Option<twitch_oauth2::UserToken>, Error> {
    match twitch_token_query::get(db).await? {
        Some(token) => {
            let Some(access_token) = token.access_token.map(|x| AccessToken::new(x)) else {
                return Ok(None);
            };
            let refresh_token = token.refresh_token.map(|x| RefreshToken::new(x));

            let token =
                twitch_oauth2::UserToken::from_existing(client, access_token, refresh_token, None)
                    .await?;

            Ok(Some(token))
        }
        _ => Ok(None),
    }
}
