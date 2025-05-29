use leptos::prelude::*;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use thiserror::Error;

pub type DiscordClient = oauth2::Client<
        oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        oauth2::StandardTokenIntrospectionResponse<
            oauth2::EmptyExtraTokenFields,
            oauth2::basic::BasicTokenType,
        >,
        oauth2::StandardRevocableToken,
        oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
        oauth2::EndpointSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointNotSet,
        oauth2::EndpointSet,
    >;

#[derive(Error, Debug)]
pub enum DiscordOAuthError {
    #[error("Failed to get env variable: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Failed to parse oauth url: {0}")]
    Url(#[from] url::ParseError),
}

pub fn from_environment() -> Result<DiscordClient, DiscordOAuthError> {
    let client_id = std::env::var("DISCORD_OAUTH_CLIENT_ID").map(|x| ClientId::new(x))?;
    let client_secret = std::env::var("DISCORD_OAUTH_CLIENT_SECRET").map(|x| ClientSecret::new(x))?;
    let auth_url = std::env::var("DISCORD_OAUTH_AUTH_URL").map(|x| AuthUrl::new(x))??;
    let token_url = std::env::var("DISCORD_OAUTH_TOKEN_URL").map(|x| TokenUrl::new(x))??;
    let redirect_url = std::env::var("DISCORD_OAUTH_REDIRECT_URL").map(|x| RedirectUrl::new(x))??;

    let client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);

    Ok(client)
}