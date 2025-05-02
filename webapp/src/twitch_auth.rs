use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;

#[derive(Params, PartialEq, Clone)]
struct TwitchConnectedParams {
    code: Option<String>,
    scope: Option<String>,
    state: Option<String>,
}

#[server]
async fn generate_token_url() -> Result<String, ServerFnError> {
    use crate::ssr::TwitchContext;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use twitch_oauth2::UserTokenBuilder;
    use twitch_oauth2::Scope;

    let twitch_context: Data<TwitchContext> = extract().await?;

    let mut builder = UserTokenBuilder::new(
        twitch_context.client_id().clone(),
        twitch_context.client_secret().clone(),
        twitch_context.redirect_url().clone(),
    )
    .set_scopes(vec![Scope::ChannelModerate, Scope::UserReadChat, Scope::ModeratorReadWarnings])
    .force_verify(true);

    let (url, _) = builder.generate_url();

    if let Ok(mut token_builder) = twitch_context.builder().lock() {
        *token_builder = Some(builder);
    }

    Ok(url.to_string())
}

#[component]
pub fn TwitchConnectPage() -> impl IntoView {
    view! {
        <Await future=generate_token_url() let:url>
            <h1>"Auth bot for twitch"</h1>
            <a href=url.clone().unwrap()>Authorize</a>
        </Await>
    }
}

#[server]
async fn generate_token(state: String, code: String) -> Result<(), ServerFnError> {
    use crate::ssr::{TwitchContext, DbContext};
    use actix_web::web::Data;
    use leptos_actix::extract;
    use sea_orm::sqlx::types::chrono::Utc;
    use sea_orm::DbConn;
    use welcome_service::twitch_broadcaster;

    let db_context: Data<DbContext> = extract().await?;
    let db = &db_context.0;

    let twitch_context: Data<TwitchContext> = extract().await?;

    // Lock the builder and extract its value before entering the async block
    let builder_option = {
        let mut builder = twitch_context.builder().lock().unwrap();
        builder.take() // Take the value out of the Mutex
    };

    println!("hi1");

    if let Some(builder) = builder_option {
        println!("hi2");
        // Use the builder to get the user token
        if let Ok(token) = builder
            .get_user_token(twitch_context.twitch_client(), &state, &code)
            .await
        {
            println!("hi6");
            if let Some(mut twitch_broadcaster) =
                twitch_broadcaster::get_by_broadcaster_id(&db, token.user_id.as_str())
                    .await?
            {
                println!("hi4");
                twitch_broadcaster.access_token = token.access_token.secret().to_string();
                twitch_broadcaster.refresh_token =
                    token.refresh_token.map(|x| x.secret().to_string());
                twitch_broadcaster::update(&db, twitch_broadcaster).await?;
            } else {
                println!("hi6");
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
pub fn TwitchConnectedPage() -> impl IntoView {
    let params = use_query::<TwitchConnectedParams>();

    let token_resource = Resource::new(move || params.get(), |params| async move {
        if let Ok(params) = params {
            println!("hi8");
            if let (Some(code), Some(state)) = (params.code, params.state) {
                println!("hi9");
                return generate_token(state, code).await;
            }
        }

        println!("hi10");

        Ok(())
    });

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <p>Success {token_resource.read().clone().unwrap().map(|_| "Yippie")}</p>
        </Transition>
    }
}
