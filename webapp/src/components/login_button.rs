use leptos::prelude::*;

#[server]
pub async fn redirect_to_oauth() -> Result<(), ServerFnError> {
    use crate::discord::oauth::DiscordClient;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_session::Session;
    use oauth2::{CsrfToken, Scope};

    let session = extract::<Session>().await?;
    let client: Data<DiscordClient> = extract().await?;

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    session.insert("CSRF_TOKEN", csrf_token)?;

    leptos_actix::redirect(auth_url.as_str());

    Ok(())
}

#[component]
pub fn LoginButton(
    #[prop(optional, into)]
    login_text: Option<String>,
    #[prop(optional, into)]
    logout_text: Option<String>,
) -> impl IntoView {
    let (button_text, set_button_text) = signal(login_text.unwrap_or("Login".to_string()));
    let redirect_action = Action::new(|_: &()| async { redirect_to_oauth().await.unwrap() });

    view! {
        <button on:click=move |ev| {
            redirect_action.dispatch(());
        }>{button_text.get()}</button>
    }
}