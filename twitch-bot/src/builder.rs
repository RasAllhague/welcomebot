use std::sync::Arc;

use sea_orm::DbConn;
use tokio::sync::Mutex;
use ttv::{
    error::Error,
    websocket::{TwitchClient, UserTokenArc},
};
use twitch_api::{HelixClient, client::ClientDefault, types::UserName};
use url::Url;

use crate::{
    bot::TtvBot,
    utils::{load_token_from_db, save_token_to_db},
};

/// A builder for creating a `TtvBot` instance.
///
/// This struct provides methods to configure and initialize a Twitch bot,
/// including setting up authentication, broadcaster logins, and database connections.
#[derive(Debug, Clone)]
pub struct TtvBotBuilder {
    /// The database connection.
    db: DbConn,
    /// A list of broadcaster logins to monitor.
    broadcaster_logins: Vec<UserName>,
    /// The bot user login name.
    bot_user_login: UserName,
}

impl TtvBotBuilder {
    /// Creates a new `TtvBotBuilder` instance.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `client_id` - The Twitch client ID.
    ///
    /// # Returns
    /// A new `TtvBotBuilder` instance.
    pub fn new(db: &DbConn, bot_user_login: UserName) -> Self {
        Self {
            db: db.clone(),
            broadcaster_logins: Vec::new(),
            bot_user_login,
        }
    }

    /// Adds a broadcaster login to the list of monitored broadcasters.
    ///
    /// # Arguments
    /// * `login` - The broadcaster's login name.
    ///
    /// # Returns
    /// The updated `TtvBotBuilder` instance.
    pub fn add_broadcaster_login(mut self, login: UserName) -> Self {
        self.broadcaster_logins.push(login);
        self
    }

    /// Builds and initializes the `TtvBot` instance.
    ///
    /// This method sets up the Twitch bot, including authentication, loading tokens,
    /// and retrieving broadcaster information.
    ///
    /// # Returns
    /// A tuple containing the `TtvBot` instance and a `Receiver` for bot events.
    ///
    /// # Errors
    /// Returns an [`Error`] if any operation fails, such as authentication or broadcaster retrieval.
    #[fastrace::trace]
    pub async fn build(self) -> Result<TtvBot, Error> {
        let client: TwitchClient = HelixClient::with_client(ClientDefault::default_client());

        let bot_token = Self::get_bot_token(&client, &self.db, &self.bot_user_login)
            .await
            .map_err(|err| ttv::error::Error::CustomError(Box::new(err)))?;
        let broadcaster_tokens =
            Self::get_broadcaster_tokens(&client, &self.db, &self.broadcaster_logins).await?;

        Ok(TtvBot {
            client,
            broadcasters: broadcaster_tokens,
            db: self.db,
            bot_token,
        })
    }

    async fn get_bot_token(
        client: &TwitchClient,
        db: &DbConn,
        bot_user_login: &UserName,
    ) -> Result<UserTokenArc, Error> {
        if let Some(token) = load_token_from_db(db, &client, bot_user_login.as_str())
            .await
            .map_err(|err| ttv::error::Error::CustomError(Box::new(err)))?
        {
            save_token_to_db(db, &token)
                .await
                .map_err(|err| ttv::error::Error::CustomError(Box::new(err)))?;

            Ok(Arc::new(Mutex::new(token)))
        } else {
            Err(Error::BotTokenNotFound(bot_user_login.clone()))
        }
    }

    async fn get_broadcaster_tokens(
        client: &TwitchClient,
        db: &DbConn,
        broadcaster_logins: &[UserName],
    ) -> Result<Vec<UserTokenArc>, Error> {
        // Retrieve broadcaster information
        let mut broadcasters = Vec::new();

        for broadcaster_login in broadcaster_logins {
            // Load or generate the authentication token
            if let Some(token) = load_token_from_db(db, &client, broadcaster_login.as_str())
                .await
                .map_err(|err| ttv::error::Error::CustomError(Box::new(err)))?
            {
                // Save the token to the database
                save_token_to_db(db, &token)
                    .await
                    .map_err(|err| ttv::error::Error::CustomError(Box::new(err)))?;

                broadcasters.push(Arc::new(Mutex::new(token)));
            }
        }

        Ok(broadcasters)
    }
}
