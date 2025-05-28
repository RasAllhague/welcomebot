use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, WildcardSegment,
};

use crate::{
    components::{Footer, Navbar},
    pages::{discord::{Discord, DiscordConnect, ModerationSettings, WelcomeSettings}, twitch::{Twitch, TwitchConnect, TwitchConnected}, Home, NotFound, Profile},
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
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/discord") view=Discord />
                    <Route path=path!("/discord/connect") view=DiscordConnect />
                    <Route path=path!("/discord/moderation") view=ModerationSettings />
                    <Route path=path!("/discord/welcome") view=WelcomeSettings />
                    <Route path=path!("/twitch") view=Twitch />
                    <Route path=path!("/profile") view=Profile />
                    <Route path=path!("/twitch/connect") view=TwitchConnect />
                    <Route path=path!("/twitch/connected") view=TwitchConnected />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
