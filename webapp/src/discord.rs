use leptos::prelude::*;

#[component]
pub fn DiscordPage() -> impl IntoView {
    view! {
        <h1>"Discord Configuration"</h1>
        <DiscordStatistics />
    }
}

#[component]
pub fn GuildSelection() -> impl IntoView {
    view! {
        <div>
            <label>"Guild: "</label>
            <select>
                <option>Guild one</option>
                <option>Guild two</option>
                <option>Guild three</option>
            </select>
        </div>
    }
}

#[component]
fn DiscordStatistics() -> impl IntoView {
    view! { <h2>"Statistics"</h2> }
}

#[component]
pub fn DiscordConnect() -> impl IntoView {
    view! {
        <h1>"Connection"</h1>

        <a href="">"Connect Discord"</a>
    }
}

#[component]
pub fn WelcomeSettings() -> impl IntoView {
    view! {
        <h1>"Welcome Settings"</h1>

        <form>
            <div>
                <label>"Chat Message: "</label>
                <input type="text" />
            </div>
            <div>
                <label>"Image Headline: "</label>
                <input type="text" />
            </div>
            <div>
                <label>"Image Subline: "</label>
                <input type="text" />
            </div>
            <div>
                <label>"Image "</label>
                <input type="file" />
            </div>
            <div>
                <label>"Channel: "</label>
                <select>
                    <option select>"Channel 1"</option>
                    <option>"Channel 2"</option>
                    <option>"Channel 3"</option>
                </select>
            </div>
            <div>
                <input type="checkbox" />
                <label>"Enabled: "</label>
            </div>
            <button>"Update"</button>
        </form>
    }
}

#[component]
pub fn ModerationSettings() -> impl IntoView {
    view! {
        <h1>"Moderation Settings"</h1>

        <form>
            <div>
                <label>"Moderation Channel: "</label>
                <select>
                    <option selected>"Channel 2"</option>
                    <option>"Channel 3"</option>
                </select>
            </div>
            <div>
                <label>"Ban reason: "</label>
                <input type="text" />
            </div>
            <div>
                <label>"Anti Bot Role: "</label>
                <select>
                    <option selected>"Role 1"</option>
                    <option>"Role 2"</option>
                </select>
            </div>
            <fieldset>
                <legend>"Mode: "</legend>
                <div>
                    <input type="radio" checked />
                    <label>"Disabled"</label>
                </div>
                <div>
                    <input type="radio" />
                    <label>"Notify"</label>
                </div>
                <div>
                    <input type="radio" />
                    <label>"Kick"</label>
                </div>
                <div>
                    <input type="radio" />
                    <label>"Ban"</label>
                </div>
            </fieldset>
            <button>"Update"</button>
        </form>
    }
}