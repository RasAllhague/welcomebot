use leptos::prelude::*;
use leptos_router::{components::Redirect, hooks::use_query_map};
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

use crate::{
    discord::{client::DiscordClient, user::CurrentUserRequest},
    model::User,
};

#[derive(Serialize, Deserialize, Clone)]
enum TokenGenerationResult {
    Success,
    ServerError(ServerFnError),
    ParamsEmpty,
}

#[server]
async fn generate_token(state: String, code: String) -> Result<Option<User>, ServerFnError> {
    use crate::discord::oauth::DiscordOAuth2Client;
    use crate::ssr::DbContext;
    use actix_session::Session;
    use actix_web::web::Data;
    use entity::web_user;
    use leptos_actix::extract;
    use oauth2::reqwest;
    use oauth2::{AuthorizationCode, CsrfToken};
    use sea_orm::sqlx::types::chrono::Utc;
    use welcome_service::web_user::create_or_update;

    let session: Session = extract().await?;
    let oauth_client: Data<DiscordOAuth2Client> = extract().await?;
    let discord_client: Data<DiscordClient> = extract().await?;
    let db_context: Data<DbContext> = extract().await?;

    if let Some(csrf_token) = session.get::<CsrfToken>("CSRF_TOKEN")? {
        if csrf_token.into_secret() != state {
            return Ok(None);
        }

        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let token_result = oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client)
            .await?;

        let user_req = CurrentUserRequest::new(token_result.access_token().secret(), 6)?;
        let user = discord_client.get_resource(user_req).await?;

        let web_user = web_user::Model {
            id: 0,
            username: user.username,
            user_id: user.id.parse()?,
            access_token: Some(token_result.access_token().secret().clone()),
            last_refresh: Some(Utc::now().naive_utc()),
            password: None,
            email: None,
            last_login_ip: None,
            twitch_broadcaster_id: None,
            create_date: Utc::now().naive_utc(),
            modify_date: None,
        };

        let new_user = create_or_update(&db_context.get_ref().0, web_user).await?;
        session.remove("CSRF_TOKEN");

        return Ok(Some(User {
            id: new_user.id,
            username: new_user.username,
            avatar: user.avatar,
            user_id: new_user.user_id,
            twitch_broadcaster_id: new_user.twitch_broadcaster_id,
        }));
    }

    Ok(None)
}

#[component]
pub fn DiscordConnect() -> impl IntoView {
    let params = use_query_map();

    let token_resource = Resource::new(
        move || (params.read().get("state"), params.read().get("code")),
        |(state, code)| async move {
            if let (Some(state), Some(code)) = (state, code) {
                generate_token(state, code).await
            } else {
                return Ok(None);
            }
        },
    );

    view! {
        <h1>"Connection"</h1>

        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="error">
                    <p>"Token generation failed! Errors: "</p>
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect::<Vec<_>>()
                        }}
                    </ul>
                </div>
            }
        }>
            <Suspense fallback=move || {
                view! { <p>"Authorizing..."</p> }
            }>
                {move || Suspend::new(async move {
                    let user = token_resource.await;

                    view! {
                        <h1>"Successfully authorized"</h1>
                        <p>
                            {token_resource
                                .await
                                .map(|x| {
                                    x.map(|y| y.username).unwrap_or(String::from("No Params set"))
                                })}
                        </p>
                        <Redirect path="/discord" />
                    }
                })}
            </Suspense>
        </ErrorBoundary>
    }
}
