use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Guild Selection:"</h1>
        <div>
            <div>
                <img />
                <a href>"Guild 1"</a>
            </div>
            <div>
                <img />
                <a href>"Guild 2"</a>
            </div>
            <div>
                <img />
                <a href>"Guild 3"</a>
            </div>
            <div>
                <img />
                <a href>"Guild 4"</a>
            </div>
        </div>
    }
}