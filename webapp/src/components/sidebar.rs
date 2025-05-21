use leptos::prelude::*;


#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidenav">
            <a href="/dashboard/twitch">"Twitch"</a>
            <a href="/dashboard/discord">"Discord"</a>
            <a href="/dashboard/help">"Help"</a>
        </div>
    }
}