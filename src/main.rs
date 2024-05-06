use ab_glyph::FontVec;
use img_gen::{error::Error, ImageGenerator};
use log::{info, warn};
use poise::serenity_prelude as serenity;
type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

pub static FIRA_SANS_BOLD: &str = "fsb";
pub static FIRA_MONO_MEDIUM: &str = "fmm";
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Data {
    image_generator: ImageGenerator,
}

#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), PoiseError> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn version(ctx: Context<'_>) -> Result<(), PoiseError> {
    let response = format!("Current running version: {VERSION}");
    ctx.say(response).await?;
    Ok(())
}

fn setup_image_generator() -> Result<ImageGenerator, Error> {
    let fira_sans_bold =
        FontVec::try_from_vec(include_bytes!("../assets/FiraSans-Bold.ttf").to_vec())?;
    let fira_mono_medium =
        FontVec::try_from_vec(include_bytes!("../assets/FiraMono-Medium.ttf").to_vec())?;

    let mut img_generator = ImageGenerator::new();
    img_generator.add_font(FIRA_SANS_BOLD, fira_sans_bold);
    img_generator.add_font(FIRA_MONO_MEDIUM, fira_mono_medium);

    Ok(img_generator)
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, PoiseError>,
    data: &Data,
) -> Result<(), PoiseError> {
    if let serenity::FullEvent::GuildMemberAddition { new_member } = event {
        info!("User joined: '{}'.", new_member.display_name());
        warn!("Profil url: '{:?}'.", new_member.avatar_url());
        warn!("Profil url: '{:?}'.", new_member.user.avatar_url());
        warn!("Profil url: '{:?}'.", new_member.user.default_avatar_url());

        if new_member.user.bot {
            warn!("Bot joined: '{}'.", new_member.display_name());

            return Ok(());
        }

        if let Some(img_url) = new_member.avatar_url() {
            info!("Joined user image url: {}", img_url);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let img_generator = setup_image_generator()?;

    dotenvy::dotenv().ok();
    let token = std::env::var("WELCOMEBOT_TOKEN").expect("Missing WELCOMEBOT_TOKEN.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), version()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    image_generator: img_generator,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

    Ok(())
}
