use leptos::prelude::*;

use crate::discord::GuildSelection;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header>
            <nav>
                <ul>
                    <li>
                        <a class="logo-item" href="/">
                            "welcomebot"
                        </a>
                    </li>
                    <li>
                        <a href="/discord">"Discord"</a>
                        <a href="/discord/moderation">"Moderation"</a>
                        <a href="/discord/welcome">"Welcome"</a>
                        <a href="/twitch/connect">"Twitch"</a>
                    </li>
                    <li>
                        <GuildSelection />
                    </li>
                    <li>
                        <a href="/profile">"Profile"</a>
                        <a href="https://discord.com/oauth2/authorize?client_id=1236977267222249512&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A3000%2Fdiscord%2Fsigned-in&scope=identify">"Sign in"</a>
                        <a href="/sign-out">"Sign out"</a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
