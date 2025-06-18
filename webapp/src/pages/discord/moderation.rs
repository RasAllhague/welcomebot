use leptos::prelude::*;

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
