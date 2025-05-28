mod connect;
mod moderation;
mod welcome;

pub use connect::DiscordConnect;
pub use moderation::ModerationSettings;
pub use welcome::WelcomeSettings;

use leptos::prelude::*;

#[component]
pub fn Discord() -> impl IntoView {
    view! {
        <h1>"Discord Configuration"</h1>
        <DiscordStatistics />
    }
}

#[component]
fn DiscordStatistics() -> impl IntoView {
    view! { <h2>"Statistics"</h2> }
}
