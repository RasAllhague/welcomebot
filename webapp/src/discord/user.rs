use serde::{Deserialize, Serialize};

use super::client::ResourceRequest;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub discriminator: String,
    pub public_flags: i32,
    pub global_name: String,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: bool,
    pub banner: Option<String>,
    pub accent_color: Option<i32>,
    pub locale: String,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: i32,
    pub premium_type: i32,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: String,
}

#[derive(Clone, Debug)]
pub struct CurrentUserRequest {
    url: url::Url,
    token: String,
}

impl CurrentUserRequest {
    pub fn new(token: &str, version: u8) -> Result<Self, url::ParseError> {
        Ok(Self {
            token: token.to_string(),
            url: url::Url::parse(&format!("https://discord.com/api/v{version}/users/@me"))?,
        })
    }
}

impl ResourceRequest for CurrentUserRequest {
    type Response = User;

    fn url(&self) -> &url::Url {
        &self.url
    }

    fn token(&self) -> &String {
        &self.token
    }

    fn build(
        &self,
        req_builder: reqwest::RequestBuilder,
    ) -> impl std::future::Future<
        Output = Result<reqwest::RequestBuilder, super::client::DiscordClientError>,
    > {
        std::future::ready(Ok(req_builder))
    }
}
