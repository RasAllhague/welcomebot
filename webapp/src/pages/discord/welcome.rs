use leptos::prelude::*;

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