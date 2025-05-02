use ttv::websocket::TwitchClient;
use twitch_oauth2::{url::Url, ClientId, ClientSecret};

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Mutex;
    use actix_files::Files;
    use actix_web::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use sea_orm::Database;
    use twitch_api::{client::ClientDefault, HelixClient};
    use webapp::app::*;
    use webapp::ssr::{TwitchContext, DbContext};

    dotenvy::dotenv().ok();
    console_error_panic_hook::set_once();

    let db_url = std::env::var("WELCOME_DATABASE_URL")
        .expect("WELCOME_DATABASE_URL is not set in .env file");
    let twitch_client_id = std::env::var("TWITCH_CLIENT_ID")
        .map(|x| ClientId::new(x))
        .expect("TWITCH_CLIENT_ID is not set in .env file");
    // let twitch_bot_login = std::env::var("TWITCH_BOT_LOGIN")
    //     .map(|x| UserName::new(x))
    //     .expect("TWITCH_BOT_LOGIN is not set in .env file");
    let twitch_client_secret = std::env::var("TWITCH_CLIENT_SECRET")
        .map(|x| ClientSecret::new(x))
        .expect("TWITCH_CLIENT_SECRET is not set in .env file");
    let redirect_url = std::env::var("REDIRECT_URL")
        .map(|s| Url::parse(&s))
        .expect("REDIRECT_URL is not set in .env file")
        .expect("REDIRECT_URL is not in a valid format");

    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to open db connection.");
    let db_context = web::Data::new(DbContext(conn));

    let twitch_client: TwitchClient = HelixClient::with_client(ClientDefault::default_client());
    let twitch_context = web::Data::new(TwitchContext::new(
        twitch_client,
        twitch_client_secret,
        twitch_client_id,
        redirect_url,
        Mutex::new(None),
    ));

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8" />
                                <meta
                                    name="viewport"
                                    content="width=device-width, initial-scale=1"
                                />
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone() />
                                <MetaTags />
                            </head>
                            <body>
                                <App />
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(db_context.clone())
            .app_data(twitch_context.clone())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use webapp::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
