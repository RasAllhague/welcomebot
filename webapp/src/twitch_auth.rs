use leptos::prelude::*;

#[server]
pub async fn generate_auth_token() -> Result<(), ServerFnError> {
    let mut builder = UserTokenBuilder::new(
        client_id.clone(),
        client_secret.clone(),
        redirect_uri.clone(),
    )
    .set_scopes(scopes.clone())
    .force_verify(true);

    Ok(())
}

#[component]
fn TwitchAuthPage() -> impl IntoView {
    view! {
        <h1>"Auth bot for twitch"</h1>
        <button>"Allow"</button>
    }
}