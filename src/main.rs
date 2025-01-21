pub mod command;
pub mod error;
mod utils;

use ab_glyph::PxScale;
use command::{settings::settings, version::version};
use entity::ban_entry;
use img_gen::{error::Error, ImageGenerator};
use log::{info, warn};
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use poise::serenity_prelude::{self as serenity, ChannelId, CreateAttachment, CreateMessage};
use tempfile::{tempdir, TempDir};
use utils::{
    image::{create_image_builder, download_avatar, setup_image_generator},
    moderation::ban_member_if_contains_autoban,
};
use welcome_service::{
    ban_entry_mutation, ban_entry_query::is_not_banned, guild_query, image_query,
    welcome_settings_query,
};

pub type PoiseError = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, PoiseError>;

pub struct Data {
    conn: DatabaseConnection,
    image_generator: ImageGenerator,
    temp_dir: TempDir,
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, PoiseError>,
    data: &Data,
) -> Result<(), PoiseError> {
    if let serenity::FullEvent::GuildMemberUpdate {
        old_if_available: _,
        new,
        event,
    } = event
    {
        let db = &data.conn;
        let guild_id: i64 = event.guild_id.into();

        if let Some(guild) = guild_query::get_by_guild_id(db, guild_id).await? {
            if let Some(member) = new {
                if is_not_banned(db, guild.id, member.user.id.into()).await? {
                    if ban_member_if_contains_autoban(ctx, &guild, member, event).await? {
                        let ban_entry = ban_entry::Model {
                            id: 0,
                            guild_id: guild.id,
                            user_id: member.user.id.into(),
                            user_name: member.display_name().to_string(),
                            reason: Some("Auto banned because of bot role.".to_string()),
                            create_user_id: framework.bot_id.into(),
                            create_date: chrono::Utc::now(),
                        };

                        ban_entry_mutation::create(db, ban_entry).await?;

                        if let Some(moderation_channel_id) = guild.moderation_channel_id {
                            let moderation_channel = ChannelId::new(moderation_channel_id as u64);
                            let message = format!(
                                "User **{}** (`{}`) was auto banned because of bot role.",
                                member.display_name(),
                                member.user.id
                            );
                            moderation_channel
                                .send_message(&ctx.http, CreateMessage::new().content(message))
                                .await?;

                            info!("Sent message to moderation channel.");
                        }
                    }
                }
            }
        }
    }

    if let serenity::FullEvent::GuildMemberAddition { new_member } = event {
        info!(
            "User joined: Id:'{}', name:'{}'.",
            new_member.user.id,
            new_member.display_name()
        );

        if new_member.user.bot {
            warn!("Bot joined: '{}'.", new_member.display_name());

            return Ok(());
        }

        let mut img_url = new_member
            .avatar_url()
            .or(new_member.user.avatar_url())
            .unwrap_or(new_member.user.default_avatar_url());

        info!("Img url: {}", img_url);

        if img_url.contains(".png") {
            img_url = new_member.user.default_avatar_url();
        }

        let file_path = download_avatar(&img_url, &data.temp_dir).await?;

        let (x, y) = (322, 64);
        let big_scale = PxScale { x: 40., y: 40. };
        let smollscale = PxScale { x: 24., y: 24. };

        let guild = ctx.http.get_guild(new_member.guild_id).await?;
        let members = guild.members(&ctx.http, None, None).await?.len();

        let db = &data.conn;

        if let Some(guild_model) = guild_query::get_by_guild_id(db, guild.id.into()).await? {
            if let Some(settings_id) = guild_model.welcome_settings_id {
                let welcome_settings =
                    match welcome_settings_query::get_one(db, settings_id).await? {
                        Some(m) => m,
                        None => return Ok(()),
                    };
                let back_image_model =
                    match image_query::get_one(db, welcome_settings.back_banner).await? {
                        Some(m) => m,
                        None => return Ok(()),
                    };
                let front_image_model =
                    match image_query::get_one(db, welcome_settings.front_banner).await? {
                        Some(m) => m,
                        None => return Ok(()),
                    };

                let image_builder = create_image_builder(
                    back_image_model.path,
                    front_image_model.path,
                    file_path,
                    &welcome_settings.image_headline,
                    &welcome_settings.image_subtext,
                    x,
                    y,
                    new_member.display_name(),
                    members,
                    big_scale,
                    smollscale,
                );
                let output_image = data.image_generator.generate(image_builder)?;

                let outfile_id = uuid::Uuid::new_v4();
                let outfile_path = data.temp_dir.path().join(format!("{}.png", outfile_id));
                output_image.save(&outfile_path)?;

                let channel = ChannelId::new(welcome_settings.welcome_channel as u64);

                let message = welcome_settings
                    .chat_message
                    .replace("{user}", &format!("<@{}>", new_member.user.id))
                    .replace("{guild_name}", &guild.name);

                let attachment = CreateAttachment::path(outfile_path).await?;
                let message = CreateMessage::new().content(message).add_file(attachment);

                channel.send_message(&ctx.http, message).await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let tmp_dir = tempdir().expect("Tempdir could not be created");

    let img_generator = setup_image_generator()?;

    dotenvy::dotenv().ok();
    let token = std::env::var("WELCOMEBOT_TOKEN").expect("Missing WELCOMEBOT_TOKEN.");
    let db_url = std::env::var("WELCOME_DATABASE_URL")
        .expect("WELCOME_DATABASE_URL is not set in .env file");

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    let conn = Database::connect(&db_url)
        .await
        .expect("Failed to open db connection.");
    Migrator::up(&conn, None)
        .await
        .expect("Failed to run migrations.");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![version(), settings()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    conn,
                    image_generator: img_generator,
                    temp_dir: tmp_dir,
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
