use leptos::prelude::*;

#[server]
async fn generate_token_url() -> Result<String, ServerFnError> {
    use crate::ssr::TwitchContext;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use twitch_oauth2::Scope;
    use twitch_oauth2::UserTokenBuilder;

    let twitch_context: Data<TwitchContext> = extract().await?;

    let mut builder = UserTokenBuilder::new(
        twitch_context.client_id().clone(),
        twitch_context.client_secret().clone(),
        twitch_context.redirect_url().clone(),
    )
    .set_scopes(vec![
        Scope::ChannelModerate,
        Scope::UserReadChat,
        Scope::ModeratorReadWarnings,
    ])
    .force_verify(true);

    let (url, _) = builder.generate_url();

    if let Ok(mut token_builder) = twitch_context.builder().lock() {
        *token_builder = Some(builder);
    }

    Ok(url.to_string())
}

#[component]
pub fn TwitchConnect() -> impl IntoView {
    view! {
        <Await future=generate_token_url() let:url>
            <h1>"Twitch linking:"</h1>
            <p>
                "Authorize the bot to connect to your twitch account. This allows the bot to collect send twitch moderation logs to discord."
            </p>
            <a href=url.clone().unwrap()>Authorize</a>
        </Await>
    }
}
