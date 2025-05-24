use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path, WildcardSegment,
};

use crate::{
    auth::{TwitchConnectPage, TwitchConnectedPage}, components::{Footer, Navbar}, discord::DiscordPage, home::HomePage, profile::ProfilePage, twitch::TwitchPage
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/webapp.css" />

        // sets the document title
        <Title text="Welcomebot" />

        // content for this welcome page
        <Router>
            <Navbar />
            <main class="content">
                <Routes fallback=move || "Not found.">
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/discord") view=DiscordPage />
                    <Route path=path!("/twitch") view=TwitchPage />
                    <Route path=path!("/profile") view=ProfilePage />
                    <Route path=path!("/twitch/connect") view=TwitchConnectPage />
                    <Route path=path!("/twitch/connected") view=TwitchConnectedPage />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
