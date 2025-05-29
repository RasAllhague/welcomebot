use leptos::prelude::*;
use leptos_router::{components::Redirect, hooks::use_query_map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
enum TokenGenerationResult {
    Success,
    ServerError(ServerFnError),
    ParamsEmpty,
}

#[server]
async fn generate_token(state: String, code: String) -> Result<(), ServerFnError> {
    use oauth2::{AuthorizationCode, CsrfToken};
    use actix_session::Session;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use oauth2::reqwest;
    use crate::discord::oauth::DiscordClient;

    let session: Session = extract().await?;
    let client: Data<DiscordClient> = extract().await?;

    if let Some(csrf_token) = session.get::<CsrfToken>("CSRF_TOKEN")? {
        if csrf_token.into_secret() != state {
            return Ok(())
        }
        
        let http_client = reqwest::ClientBuilder::new().redirect(reqwest::redirect::Policy::none())
            .build()?;
        
        let token_result = client.exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client).await?;

        session.remove("CSRF_TOKEN");
    } 

    Ok(())
}

#[component]
pub fn DiscordConnect() -> impl IntoView {
    let params = use_query_map();

    let token_resource = Resource::new(
        move || (params.read().get("state"), params.read().get("code")),
        |(state, code)| async move {
            if let (Some(state), Some(code)) = (state, code) {
                if let Err(why) = generate_token(state, code).await {
                    return TokenGenerationResult::ServerError(why);
                }
                else {
                    return TokenGenerationResult::Success;
                }
            } else {
                return TokenGenerationResult::ParamsEmpty;
            }
        },
    );

    view! {
        <h1>"Connection"</h1>

        <Suspense fallback=move || {
            view! { <p>"Authorizing..."</p> }
        }>
            {move || Suspend::new(async move {
                match token_resource.await.clone() {
                    TokenGenerationResult::Success => {
                        view! {
                            <h1>"Successfully authorized"</h1>
                            <Redirect path="/discord" />
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
