use leptos::prelude::*;
use url::Url;

#[derive(Clone, Debug)]
pub struct DiscordOauth2(Url);

impl DiscordOauth2 {
    pub fn new(url: Url) -> Self {
        Self(url)
    }
}