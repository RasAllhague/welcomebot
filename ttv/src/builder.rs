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

#[derive(Debug, Clone, Default)]
pub enum AuthWorkflow {
    /// The bot will use the device code flow to authenticate.
    #[default]
    DeviceCode,
    /// The bot will use the authorization code flow to authenticate.
    AuthorizationCode,
}

#[derive(Debug, Clone)]
pub struct TtvBotBuilder {
    db: DbConn,
    client_id: twitch_oauth2::ClientId,
    broadcaster_logins: Vec<twitch_api::types::UserName>,
    auth_workflow: AuthWorkflow,
}

impl TtvBotBuilder {
    pub fn new(db: &DbConn, client_id: twitch_oauth2::ClientId) -> Self {
        Self {
            db: db.clone(),
            client_id,
            broadcaster_logins: Vec::new(),
            auth_workflow: AuthWorkflow::default(),
        }
    }

    pub fn add_broadcaster_login(mut self, login: twitch_api::types::UserName) -> Self {
        self.broadcaster_logins.push(login);
        self
    }

    pub fn set_device_flow(mut self) -> Self {
        self.auth_workflow = AuthWorkflow::DeviceCode;
        self
    }

    pub fn set_authorization_code_flow(mut self) -> Self {
        self.auth_workflow = AuthWorkflow::AuthorizationCode;
        self
    }

    #[fastrace::trace]
    pub async fn build(self) -> Result<(TtvBot, Receiver<BotEvent>), Error> {
        let client: HelixClient<reqwest::Client> =
            HelixClient::with_client(ClientDefault::default_client()); // maybe use default_client_with_name

        let token = if let Some(token) = load_token_from_db(&self.db, &client).await? {
            token
        } else {
            match self.auth_workflow {
                AuthWorkflow::DeviceCode => self.get_device_token(&client).await?,
                AuthWorkflow::AuthorizationCode => todo!("Authorization code flow not done yet"),
            }
        };

        save_token_to_db(&self.db, &token).await?;

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
                sender: sender,
            },
            receiver,
        ))
    }

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
