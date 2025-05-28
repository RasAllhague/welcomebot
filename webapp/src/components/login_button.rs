use leptos::prelude::*;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope, TokenUrl};

#[component]
pub fn LoginButton(
    #[prop(optional, into)]
    login_text: Option<String>,
    #[prop(optional, into)]
    logout_text: Option<String>,
) -> impl IntoView {
    let (button_text, set_button_text) = signal(login_text.unwrap_or("Login".to_string()));
    let redirect_action = Action::new(|_: &()| async { redirect_to_oauth().await.unwrap() });

    view! { <button on:click=move |ev| {
        redirect_action.dispatch(());
    }>{button_text.get()}</button> }
}

#[server]
pub async fn redirect_to_oauth() -> Result<(), ServerFnError> {
    let client = BasicClient::new(ClientId::new("1236977267222249512".to_string()))
        .set_client_secret(ClientSecret::new("client_secret".to_string()))
        .set_auth_uri(AuthUrl::new("https://discord.com/oauth2/authorize".to_string())?)
        .set_token_uri(TokenUrl::new("https://discord.com/api/oauth2/token".to_string())?)
        .set_redirect_uri(RedirectUrl::new("http://localhost:3000/discord/signed-in".to_string())?);

    let (pkce_challenge, pcfe_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    Ok(())
}