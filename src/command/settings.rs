use chrono::Utc;
use entity::guild;
use log::error;
use migration::sea_orm::DbConn;
use poise::serenity_prelude::{self as serenity, CreateMessage, Guild};
use welcome_service::{guild_mutation, guild_query};

use crate::Data;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// Settings of welcome bot. With this you can update its behaviour.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "The text of the welcome message"] message: Option<String>,
    #[description = "The channel where to send welcome messages to"]
    #[channel_types("Text")]
    channel: Option<serenity::Channel>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    // unwrap since we are in a guild only command
    let guild = ctx.guild().unwrap().clone();
    let author_id = ctx.author().id.into();

    if let Some(channel) = channel {
        if let Err(why) = update_welcome_channel(db, &guild, author_id, channel).await {
            error!("Could not update welcome channel: {why}");

            ctx.channel_id()
                .send_message(
                    &ctx,
                    CreateMessage::new()
                        .content(format!("Could not update welcome channel.")),
                )
                .await?;
        }
    }
    if let Some(message) = message {
        if let Err(why) = update_welcome_message(db, &guild, author_id, &message).await {
            error!("Could not update welcome message: {why}");

            ctx.channel_id()
                .send_message(
                    &ctx,
                    CreateMessage::new()
                        .content(format!("Could not update welcome message.")),
                )
                .await?;
        }
    }

    ctx.say("Finished updating.").await?;

    Ok(())
}

async fn update_welcome_channel(
    db: &DbConn,
    discord_guild: &Guild,
    create_user_id: i64,
    channel: serenity::Channel,
) -> Result<(), PoiseError> {
    let guild_id = discord_guild.id.into();

    if let Some(mut guild) = guild_query::get_by_guild_id(db, guild_id).await? {
        guild.welcome_channel = Some(channel.id().into());
        guild.modify_date = Some(Utc::now().naive_utc().to_string());
        guild.modify_user_id = Some(create_user_id);

        guild_mutation::update(db, guild).await?;
    } else {
        let guild = guild::Model {
            id: 0,
            name: discord_guild.name.clone(),
            guild_id,
            welcome_message: None,
            welcome_channel: Some(channel.id().into()),
            back_banner: 1,
            front_banner: 1,
            create_user_id: create_user_id,
            create_date: Utc::now().naive_utc().to_string(),
            modify_date: None,
            modify_user_id: None,
        };

        guild_mutation::create(db, guild).await?;
    }

    Ok(())
}

async fn update_welcome_message(
    db: &DbConn,
    discord_guild: &Guild,
    create_user_id: i64,
    welcome_message: &str,
) -> Result<(), PoiseError> {
    let guild_id = discord_guild.id.into();

    if let Some(mut guild) = guild_query::get_by_guild_id(db, guild_id).await? {
        guild.welcome_message = Some(welcome_message.to_owned());
        guild.modify_date = Some(Utc::now().naive_utc().to_string());
        guild.modify_user_id = Some(create_user_id);

        guild_mutation::update(db, guild).await?;
    } else {
        let guild = guild::Model {
            id: 0,
            name: discord_guild.name.clone(),
            guild_id,
            welcome_message: Some(welcome_message.to_owned()),
            welcome_channel: None,
            back_banner: 1,
            front_banner: 1,
            create_user_id: create_user_id,
            create_date: Utc::now().naive_utc().to_string(),
            modify_date: None,
            modify_user_id: None,
        };

        guild_mutation::create(db, guild).await?;
    }

    Ok(())
}
