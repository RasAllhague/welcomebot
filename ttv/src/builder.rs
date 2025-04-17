use std::sync::Arc;

use crossbeam_channel::Receiver;
use sea_orm::DbConn;
use tokio::sync::Mutex;
use twitch_api::{client::ClientDefault, types::UserName, HelixClient};
use twitch_oauth2::{ClientId, ClientSecret, Scope};
use url::Url;

use crate::{
    auth::AuthWorkflow,
    bot::TtvBot,
    error::Error,
    queue::BotEvent,
    utils::{load_token_from_db, save_token_to_db},
    websocket::{TwitchClient, UserTokenArc},
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
    /// The authentication workflow to use.
    auth_workflow: AuthWorkflow,
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
    pub fn new(db: &DbConn, client_id: ClientId, bot_user_login: UserName) -> Self {
        Self {
            db: db.clone(),
            broadcaster_logins: Vec::new(),
            auth_workflow: AuthWorkflow::DeviceCode {
                client_id,
                scopes: Scope::all(),
            },
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

    /// Sets the authentication workflow to use the device code flow.
    ///
    /// # Returns
    /// The updated `TtvBotBuilder` instance.
    pub fn set_device_flow(mut self, client_id: ClientId, scopes: Vec<Scope>) -> Self {
        self.auth_workflow = AuthWorkflow::DeviceCode { client_id, scopes };
        self
    }

    /// Sets the authentication workflow to use the authorization code flow.
    ///
    /// # Returns
    /// The updated `TtvBotBuilder` instance.
    pub fn set_authorization_code_flow(
        mut self,
        client_id: ClientId,
        client_secret: ClientSecret,
        scopes: Vec<Scope>,
        redirect_url: Url,
    ) -> Self {
        self.auth_workflow = AuthWorkflow::AuthorizationCode {
            client_id,
            scopes,
            client_secret,
            redirect_url,
        };
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
    pub async fn build(self) -> Result<(TtvBot, Receiver<BotEvent>), Error> {
        let client: TwitchClient = HelixClient::with_client(ClientDefault::default_client());

        let bot_token = Self::get_bot_token(&client, &self.db, &self.bot_user_login).await?;
        let broadcaster_tokens = Self::get_broadcaster_tokens(
            &client,
            &self.db,
            &self.auth_workflow,
            &self.broadcaster_logins,
        )
        .await?;

        let (sender, receiver) = crossbeam_channel::unbounded::<BotEvent>();

        Ok((
            TtvBot {
                client,
                broadcasters: broadcaster_tokens,
                db: self.db,
                sender,
                bot_token,
            },
            receiver,
        ))
    }

    async fn get_bot_token(
        client: &TwitchClient,
        db: &DbConn,
        bot_user_login: &UserName,
    ) -> Result<UserTokenArc, Error> {
        if let Some(token) = load_token_from_db(db, &client, bot_user_login.as_str()).await? {
            save_token_to_db(db, &token).await?;

            Ok(Arc::new(Mutex::new(token)))
        } else {
            Err(Error::BotTokenNotFound(bot_user_login.clone()))
        }
    }

    async fn get_broadcaster_tokens(
        client: &TwitchClient,
        db: &DbConn,
        auth_workflow: &AuthWorkflow,
        broadcaster_logins: &[UserName],
    ) -> Result<Vec<UserTokenArc>, Error> {
        // Retrieve broadcaster information
        let mut broadcasters = Vec::new();

        for broadcaster_login in broadcaster_logins {
            // Load or generate the authentication token
            let token = if let Some(token) =
                load_token_from_db(db, &client, broadcaster_login.as_str()).await?
            {
                token
            } else {
                auth_workflow.get_token(&client).await?
            };

            // Save the token to the database
            save_token_to_db(db, &token).await?;

            broadcasters.push(Arc::new(Mutex::new(token)));
        }

        Ok(broadcasters)
    }
}
