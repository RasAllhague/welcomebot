use leptos::prelude::*;
use url::Url;

#[derive(Clone, Debug)]
pub struct DiscordOauth2(Url);

impl DiscordOauth2 {
    pub fn new(url: Url) -> Self {
        Self(url)
    }
}

#[server]
pub async fn generate_oauth2_url() -> Result<String, ServerFnError> {
    todo!()
}
