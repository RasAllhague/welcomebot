use std::sync::Arc;

use crossbeam_channel::Receiver;
use sea_orm::DbConn;
use tokio::sync::Mutex;
use twitch_api::{client::ClientDefault, helix::users::User, HelixClient};

use crate::{
    bot::{TtvBot, SCOPES},
    error::Error,
    queue::BotEvent,
    utils::{load_token_from_db, save_token_to_db},
};

/// Represents the authentication workflow for the Twitch bot.
///
/// This enum defines the two possible authentication methods:
/// - `DeviceCode`: Uses the device code flow for authentication.
/// - `AuthorizationCode`: Uses the authorization code flow for authentication.
#[derive(Debug, Clone, Default)]
pub enum AuthWorkflow {
    /// The bot will use the device code flow to authenticate.
    #[default]
    DeviceCode,
    /// The bot will use the authorization code flow to authenticate.
    AuthorizationCode,
}

/// A builder for creating a `TtvBot` instance.
///
/// This struct provides methods to configure and initialize a Twitch bot,
/// including setting up authentication, broadcaster logins, and database connections.
#[derive(Debug, Clone)]
pub struct TtvBotBuilder {
    /// The database connection.
    db: DbConn,
    /// The Twitch client ID.
    client_id: twitch_oauth2::ClientId,
    /// A list of broadcaster logins to monitor.
    broadcaster_logins: Vec<twitch_api::types::UserName>,
    /// The authentication workflow to use.
    auth_workflow: AuthWorkflow,
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
    pub fn new(db: &DbConn, client_id: twitch_oauth2::ClientId) -> Self {
        Self {
            db: db.clone(),
            client_id,
            broadcaster_logins: Vec::new(),
            auth_workflow: AuthWorkflow::default(),
        }
    }

    /// Adds a broadcaster login to the list of monitored broadcasters.
    ///
    /// # Arguments
    /// * `login` - The broadcaster's login name.
    ///
    /// # Returns
    /// The updated `TtvBotBuilder` instance.
    pub fn add_broadcaster_login(mut self, login: twitch_api::types::UserName) -> Self {
        self.broadcaster_logins.push(login);
        self
    }

    /// Sets the authentication workflow to use the device code flow.
    ///
    /// # Returns
    /// The updated `TtvBotBuilder` instance.
    pub fn set_device_flow(mut self) -> Self {
        self.auth_workflow = AuthWorkflow::DeviceCode;
        self
    }

    /// Sets the authentication workflow to use the authorization code flow.
    ///
    /// # Returns
    /// The updated `TtvBotBuilder` instance.
    pub fn set_authorization_code_flow(mut self) -> Self {
        self.auth_workflow = AuthWorkflow::AuthorizationCode;
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
        let client: HelixClient<reqwest::Client> =
            HelixClient::with_client(ClientDefault::default_client());

        // Load or generate the authentication token
        let token = if let Some(token) = load_token_from_db(&self.db, &client).await? {
            token
        } else {
            match self.auth_workflow {
                AuthWorkflow::DeviceCode => self.get_device_token(&client).await?,
                AuthWorkflow::AuthorizationCode => todo!("Authorization code flow not implemented yet"),
            }
        };

        // Save the token to the database
        save_token_to_db(&self.db, &token).await?;

        // Retrieve broadcaster information
        let mut broadcasters = Vec::new();
        for login in &self.broadcaster_logins {
            let Some(User { id, .. }) = client.get_user_from_login(login, &token).await? else {
                return Err(Error::BroadcasterNotFound(login.to_string()));
            };

            broadcasters.push(id);
        }

        let token = Arc::new(Mutex::new(token));
        let (sender, receiver) = crossbeam_channel::unbounded::<BotEvent>();

        Ok((
            TtvBot {
                client,
                token,
                broadcasters,
                db: self.db,
                sender,
            },
            receiver,
        ))
    }

    /// Retrieves a device token using the device code flow.
    ///
    /// This method prompts the user to visit a URL and enter a code to authenticate.
    ///
    /// # Arguments
    /// * `client` - The Helix client for interacting with the Twitch API.
    ///
    /// # Returns
    /// A `UserToken` containing the authentication token.
    ///
    /// # Errors
    /// Returns an [`Error`] if the device code flow fails.
    async fn get_device_token(
        &self,
        client: &HelixClient<'_, reqwest::Client>,
    ) -> Result<twitch_oauth2::UserToken, Error> {
        let mut builder = twitch_oauth2::tokens::DeviceUserTokenBuilder::new(
            self.client_id.clone(),
            SCOPES.to_vec(),
        );
        let code = builder.start(client).await?;

        println!("Please go to: {}", code.verification_uri);

        Ok(builder.wait_for_code(client, tokio::time::sleep).await?)
    }
}
