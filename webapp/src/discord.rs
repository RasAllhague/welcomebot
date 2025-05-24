use leptos::prelude::*;


#[component]
pub fn DiscordPage() -> impl IntoView {
    view! {
        <h1>"Discord Configuration"</h1>
        <GuildSelection />
        <DiscordStatistics />
        <hr />
        <DiscordConnect />
        <hr />
        <WelcomeSettings />
        <hr />
        <ModerationSettings />
    }
}

#[component]
fn GuildSelection() -> impl IntoView {
    view! {
        <p>Guild:</p>
        <select>
            <option>Guild one</option>
            <option>Guild two</option>
            <option>Guild three</option>
        </select>
    }
}

#[component]
fn DiscordStatistics() -> impl IntoView {
    view! { <h2>"Statistics"</h2> }
}

#[component]
fn DiscordConnect() -> impl IntoView {
    view! {
        <h2>"Connection"</h2>
        <a href="">"Connect Discord"</a>
    }
}

#[component]
fn WelcomeSettings() -> impl IntoView {
    view! {
        <h2>"Welcome Settings"</h2>

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
        </form>
    }
}

#[component]
fn ModerationSettings() -> impl IntoView {
    view! {
        <h2>"Moderation Settings"</h2>
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
        </form>
    }
}