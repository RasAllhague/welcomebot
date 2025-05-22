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
                        <a href="/discord">"Discord"</a>
                    </li>
                    <li>
                        <a href="/twitch">"Twitch"</a>
                    </li>
                    <li>
                        <a href="/profile">"Profile"</a>
                    </li>
                    <li>
                        <a href="/sign-in">"Sign in"</a>
                        <a href="/sign-out">"Sign out"</a>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
