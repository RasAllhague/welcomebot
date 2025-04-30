pub mod app;
mod components;
mod twitch_auth;

pub use twitch_auth::TwitchConnectPage;

#[cfg(feature = "ssr")]
pub mod ssr {
    use sea_orm::DbConn;
    use ttv::websocket::TwitchClient;
    use twitch_oauth2::{ClientId, ClientSecret};
    use url::Url;

    #[derive(Clone)]
    pub struct DbContext(pub DbConn);
    
    #[derive(Clone)]
    pub struct TwitchContext {
        twitch_client: TwitchClient,
        client_secret: ClientSecret,
        client_id: ClientId,
        redirect_url: Url,
    }

    impl TwitchContext {
        pub fn new(
            twitch_client: TwitchClient,
            client_secret: ClientSecret,
            client_id: ClientId,
            redirect_url: Url,
        ) -> Self {
            Self {
                twitch_client,
                client_secret,
                client_id,
                redirect_url,
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
    }
}


#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
