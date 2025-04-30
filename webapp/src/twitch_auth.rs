use leptos::prelude::*;

#[server]
pub async fn generate_auth_token() -> Result<String, ServerFnError> {
    use crate::ssr::TwitchContext;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use twitch_oauth2::UserTokenBuilder;

    let twitch_context: Data<TwitchContext> = extract().await?;

    let mut builder = UserTokenBuilder::new(
        twitch_context.client_id().clone(),
        twitch_context.client_secret().clone(),
        twitch_context.redirect_url().clone(),
    )
    .set_scopes(vec![])
    .force_verify(true);

    let (url, csrf) = builder.generate_url();

    Ok(url.to_string())
}

#[component]
pub fn TwitchConnectPage() -> impl IntoView {
    view! {
        <Await
        future=generate_auth_token()
        let:url
        >
            <h1>"Auth bot for twitch"</h1>
            <a href={url.clone().unwrap()}>Authorize</a>
        </Await>
    }
}

#[component]
pub fn TwitchConnectedPage() -> impl IntoView {

}
