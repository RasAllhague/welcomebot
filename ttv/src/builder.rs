use std::sync::Arc;

use crossbeam_channel::Receiver;
use sea_orm::DbConn;
use tokio::sync::Mutex;
use twitch_api::{client::ClientDefault, helix::users::User, HelixClient};
use twitch_oauth2::{ClientSecret, Scope};
use url::Url;

use crate::{
    auth::AuthWorkflow,
    bot::TtvBot,
    error::Error,
    queue::BotEvent,
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
            broadcaster_logins: Vec::new(),
            auth_workflow: AuthWorkflow::DeviceCode {
                client_id,
                scopes: Scope::all(),
            },
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
    pub fn set_device_flow(
        mut self,
        client_id: twitch_oauth2::ClientId,
        scopes: Vec<Scope>,
    ) -> Self {
        self.auth_workflow = AuthWorkflow::DeviceCode { client_id, scopes };
        self
    }

    /// Sets the authentication workflow to use the authorization code flow.
    ///
    /// # Returns
    /// The updated `TtvBotBuilder` instance.
    pub fn set_authorization_code_flow(
        mut self,
        client_id: twitch_oauth2::ClientId,
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
        let client: HelixClient<reqwest::Client> =
            HelixClient::with_client(ClientDefault::default_client());

        // Load or generate the authentication token
        let token = if let Some(token) = load_token_from_db(&self.db, &client).await? {
            token
        } else {
            self.auth_workflow.get_token(&client).await?
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
}
