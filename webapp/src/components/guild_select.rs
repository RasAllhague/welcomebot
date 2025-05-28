use leptos::prelude::*;

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
