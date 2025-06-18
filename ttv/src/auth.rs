use twitch_api::HelixClient;
use twitch_oauth2::{
    ClientId, ClientSecret, DeviceUserTokenBuilder, Scope, UserToken, UserTokenBuilder,
};
use url::Url;

use crate::error::Error;

/// Represents the authentication workflow for the Twitch bot.
///
/// This enum defines the two possible authentication methods:
/// - `DeviceCode`: Uses the device code flow for authentication.
/// - `AuthorizationCode`: Uses the authorization code flow for authentication.
#[derive(Debug, Clone)]
pub enum AuthWorkflow {
    /// The bot will use the device code flow to authenticate.
    ///
    /// # Fields
    /// - `client_id`: The Twitch client ID.
    /// - `scopes`: The list of scopes required for the bot's functionality.
    DeviceCode {
        client_id: ClientId,
        scopes: Vec<Scope>,
    },
    /// The bot will use the authorization code flow to authenticate.
    ///
    /// # Fields
    /// - `client_id`: The Twitch client ID.
    /// - `scopes`: The list of scopes required for the bot's functionality.
    /// - `client_secret`: The Twitch client secret.
    /// - `redirect_url`: The redirect URL for the authorization code flow.
    AuthorizationCode {
        client_id: ClientId,
        scopes: Vec<Scope>,
        client_secret: ClientSecret,
        redirect_url: Url,
    },
}

impl AuthWorkflow {
    /// Retrieves an authentication token based on the selected workflow.
    ///
    /// This method determines the authentication method (device code or authorization code)
    /// and retrieves the corresponding token.
    ///
    /// # Arguments
    /// * `client` - The Helix client for interacting with the Twitch API.
    ///
    /// # Returns
    /// A `UserToken` containing the authentication token.
    ///
    /// # Errors
    /// Returns an [`Error`] if the authentication process fails.
    pub async fn get_token(
        &self,
        client: &HelixClient<'_, reqwest::Client>,
    ) -> Result<UserToken, Error> {
        match self {
            AuthWorkflow::DeviceCode { client_id, scopes } => {
                Self::get_device_token(client, client_id, scopes).await
            }
            AuthWorkflow::AuthorizationCode {
                client_id,
                scopes,
                client_secret,
                redirect_url,
            } => {
                Self::get_auth_flow_token(client, client_id, scopes, client_secret, redirect_url)
                    .await
            }
        }
    }

    /// Retrieves a device token using the device code flow.
    ///
    /// This method prompts the user to visit a URL and enter a code to authenticate.
    ///
    /// # Arguments
    /// * `client` - The Helix client for interacting with the Twitch API.
    /// * `client_id` - The Twitch client ID.
    /// * `scopes` - The list of scopes required for the bot's functionality.
    ///
    /// # Returns
    /// A `UserToken` containing the authentication token.
    ///
    /// # Errors
    /// Returns an [`Error`] if the device code flow fails.
    async fn get_device_token(
        client: &HelixClient<'_, reqwest::Client>,
        client_id: &ClientId,
        scopes: &Vec<Scope>,
    ) -> Result<UserToken, Error> {
        let mut builder = DeviceUserTokenBuilder::new(client_id.clone(), scopes.clone());
        let code = builder.start(client).await?;

        println!("Please go to: {}", code.verification_uri);

        Ok(builder.wait_for_code(client, tokio::time::sleep).await?)
    }

    /// Retrieves an authentication token using the authorization code flow.
    ///
    /// This method prompts the user to authenticate via a URL and provides a token
    /// after successful authentication.
    ///
    /// # Arguments
    /// * `client` - The Helix client for interacting with the Twitch API.
    /// * `client_id` - The Twitch client ID.
    /// * `scopes` - The list of scopes required for the bot's functionality.
    /// * `client_secret` - The Twitch client secret.
    /// * `redirect_uri` - The redirect URL for the authorization code flow.
    ///
    /// # Returns
    /// A `UserToken` containing the authentication token.
    ///
    /// # Errors
    /// Returns an [`Error`] if the authorization code flow fails.
    async fn get_auth_flow_token(
        client: &HelixClient<'_, reqwest::Client>,
        client_id: &ClientId,
        scopes: &Vec<Scope>,
        client_secret: &ClientSecret,
        redirect_uri: &Url,
    ) -> Result<UserToken, Error> {
        let mut builder = UserTokenBuilder::new(
            client_id.clone(),
            client_secret.clone(),
            redirect_uri.clone(),
        )
        .set_scopes(scopes.clone())
        .force_verify(true);
        let (url, _) = builder.generate_url();

        println!("Please go to this page: {}", url);

        let input = rpassword::prompt_password(
            "Paste in the resulting address after authenticating (input hidden): ",
        )?;
        let url = Url::parse(&input)?;

        let map: std::collections::HashMap<_, _> = url.query_pairs().collect();

        match (map.get("state"), map.get("code")) {
            (Some(state), Some(code)) => {
                let token = builder.get_user_token(client, state, code).await?;
                return Ok(token);
            }
            _ => match (map.get("error"), map.get("error_description")) {
                (Some(error), Some(error_description)) => {
                    return Err(Error::TwitchOAuthFailure {
                        error: error.to_string(),
                        error_description: error_description.to_string(),
                    })
                }
                _ => return Err(Error::InvalidOAuthUrl(url.to_string())),
            },
        }
    }
}
