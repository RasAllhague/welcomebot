use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>Hallo</h1>
        <a href="dashboard">"Zum dashboard"</a>
    }
}