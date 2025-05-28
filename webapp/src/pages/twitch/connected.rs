use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_query_map;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub enum TokenGenerationResult {
    Success,
    Failed {
        error: String,
        error_description: String,
    },
    ServerError(ServerFnError),
    ParamsEmpty,
}

#[server]
async fn generate_token(state: String, code: String) -> Result<(), ServerFnError> {
    use crate::ssr::{DbContext, TwitchContext};
    use actix_web::web::Data;
    use leptos_actix::extract;
    use sea_orm::sqlx::types::chrono::Utc;
    use welcome_service::twitch_broadcaster;

    let db_context: Data<DbContext> = extract().await?;
    let db = &db_context.0;

    let twitch_context: Data<TwitchContext> = extract().await?;

    // Lock the builder and extract its value before entering the async block
    let builder_option = {
        let mut builder = twitch_context.builder().lock().unwrap();
        builder.take() // Take the value out of the Mutex
    };

    if let Some(builder) = builder_option {
        // Use the builder to get the user token
        if let Ok(token) = builder
            .get_user_token(twitch_context.twitch_client(), &state, &code)
            .await
        {
            if let Some(mut twitch_broadcaster) =
                twitch_broadcaster::get_by_broadcaster_id(&db, token.user_id.as_str()).await?
            {
                twitch_broadcaster.access_token = token.access_token.secret().to_string();
                twitch_broadcaster.refresh_token =
                    token.refresh_token.map(|x| x.secret().to_string());
                twitch_broadcaster::update(&db, twitch_broadcaster).await?;
            } else {
                let twitch_broadcaster = entity::twitch_broadcaster::Model {
                    id: 0,
                    broadcaster_login: token.login.to_string(),
                    broadcaster_id: token.user_id.to_string(),
                    broadcaster_name: token.login.to_string(),
                    access_token: token.access_token.secret().to_string(),
                    refresh_token: token.refresh_token.map(|x| x.secret().to_string()),
                    last_refreshed: None,
                    create_date: Utc::now(),
                    modify_date: None,
                };

                twitch_broadcaster::create(&db, twitch_broadcaster).await?;
            }
        }

        // Optionally, put the builder back into the Mutex if needed
        let mut builder = twitch_context.builder().lock().unwrap();
        *builder = None;
    }

    Ok(())
}

#[component]
pub fn TwitchConnected() -> impl IntoView {
    let params = use_query_map();

    let token_resource = Resource::new(
        move || {
            (
                params.read().get("state"),
                params.read().get("code"),
                params.read().get("error_description"),
                params.read().get("error"),
            )
        },
        |(state, code, error_description, error)| async move {
            if let (Some(state), Some(code)) = (state, code) {
                match generate_token(state, code).await {
                    Ok(_) => TokenGenerationResult::Success,
                    Err(why) => TokenGenerationResult::ServerError(why),
                }
            } else if let (Some(error), Some(error_description)) = (error, error_description) {
                TokenGenerationResult::Failed {
                    error,
                    error_description,
                }
            } else {
                TokenGenerationResult::ParamsEmpty
            }
        },
    );

    view! {
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || Suspend::new(async move {
                match token_resource.await.clone() {
                    TokenGenerationResult::Success => {
                        view! {
                            <h1>"Successfully authorized"</h1>
                            <Redirect path="/dashboard" />
                        }
                            .into_any()
                    }
                    TokenGenerationResult::Failed { error, error_description } => {
                        view! {
                            <h1>"Authorization failed"</h1>
                            <p>"Error: "{error}</p>
                            <p>{error_description}</p>
                        }
                            .into_any()
                    }
                    TokenGenerationResult::ServerError(server_fn_error) => {
                        view! { <h1>"Server error: "{server_fn_error.to_string()}</h1> }.into_any()
                    }
                    TokenGenerationResult::ParamsEmpty => {
                        view! { <h1>"No params where present?"</h1> }.into_any()
                    }
                }
            })}
        </Suspense>
    }
}
