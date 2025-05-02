use leptos::prelude::*;

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
                        <a href="/twitch">"Twitch"</a>
                        <a href="/twitch/connect">"Connect"</a>
                    </li>
                    <li>
                        <a href="/discord">"Discord"</a>
                        <a href="/discord/connect">"Connect"</a>
                    </li>
                    <li>
                        <a href="/profile">"Profile"</a>
                    </li>
                    <li>
                        <a href="/sign-in">"Sign in"</a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}