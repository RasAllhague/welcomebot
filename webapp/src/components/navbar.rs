use leptos::prelude::*;

use crate::components::{GuildSelection, LoginButton};

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
                        <LoginButton />
                        <a href="/sign-out">"Sign out"</a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
