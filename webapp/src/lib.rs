pub mod app;
pub mod discord;
mod components;
mod pages;
mod model;

#[cfg(feature = "ssr")]
pub mod ssr {
    use std::sync::Mutex;

    use sea_orm::DbConn;
    use ttv::websocket::TwitchClient;
    use twitch_oauth2::{ClientId, ClientSecret, UserTokenBuilder};
    use url::Url;

    #[derive(Clone)]
    pub struct DbContext(pub DbConn);

    pub struct TwitchContext {
        twitch_client: TwitchClient,
        client_secret: ClientSecret,
        client_id: ClientId,
        redirect_url: Url,
        builder: Mutex<Option<UserTokenBuilder>>,
    }

    impl TwitchContext {
        pub fn new(
            twitch_client: TwitchClient,
            client_secret: ClientSecret,
            client_id: ClientId,
            redirect_url: Url,
            builder: Mutex<Option<UserTokenBuilder>>,
        ) -> Self {
            Self {
                twitch_client,
                client_secret,
                client_id,
                redirect_url,
                builder,
            }
        }

        pub fn twitch_client(&self) -> &TwitchClient {
            &self.twitch_client
        }

        pub fn client_secret(&self) -> &ClientSecret {
            &self.client_secret
        }

        pub fn client_id(&self) -> &ClientId {
            &self.client_id
        }

        pub fn redirect_url(&self) -> &Url {
            &self.redirect_url
        }

        pub fn builder(&self) -> &Mutex<Option<UserTokenBuilder>> {
            &self.builder
        }
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
