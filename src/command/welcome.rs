use chrono::Utc;
use entity::guild;
use migration::sea_orm::DbConn;
use poise::{
    serenity_prelude::{self as serenity},
    CreateReply,
};
use welcome_service::{guild_mutation, welcome_settings_mutation, welcome_settings_query};

use crate::{Context, PoiseError};

/// Commands for welcoming a user with the welcome bot
#[fastrace::trace]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR",
    subcommands("settings")
)]
pub async fn welcome(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Settings of welcome bot. With this you can update its behaviour.
#[fastrace::trace]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
async fn settings(
    ctx: Context<'_>,
    #[description = "The text of the chat welcome message. Placeholders: {user}, {guild_name}"]
    chat_message: Option<String>,
    #[description = "The text of the healine of the image. Placeholders: {name}"]
    image_headline: Option<String>,
    #[description = "The text of the subline of the image. Placeholders: {members}"] image_subline: Option<String>,
    #[description = "The channel where to send welcome messages to"]
    #[channel_types("Text")]
    channel: Option<serenity::Channel>,
    #[description = "Enables or disables the welcome message sending"] enabled: Option<bool>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    // unwrap since we are in a guild only command
    let discord_guild = ctx.guild().unwrap().clone();
    let author_id = ctx.author().id.into();

    let guild =
        guild_mutation::get_or_create(db, discord_guild.id.into(), discord_guild.name, author_id)
            .await?;
    update_welcome_settings(
        db,
        guild,
        author_id,
        chat_message,
        image_headline,
        image_subline,
        enabled,
        channel.map(|x| x.id()).or(discord_guild.system_channel_id),
    )
    .await?;

    ctx.send(
        CreateReply::default()
            .content("Settings updated.")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

#[fastrace::trace]
async fn update_welcome_settings(
    db: &DbConn,
    mut guild: guild::Model,
    create_user_id: i64,
    chat_message: Option<String>,
    image_headline: Option<String>,
    image_subline: Option<String>,
    enabled: Option<bool>,
    channel: Option<serenity::ChannelId>,
) -> Result<guild::Model, PoiseError> {
    if let Some(mut welcome_settings) = welcome_settings_query::get_one(db, guild.id).await? {
        welcome_settings.welcome_channel = match channel {
            Some(c) => c.into(),
            None => welcome_settings.welcome_channel,
        };
        welcome_settings.chat_message = chat_message.unwrap_or(welcome_settings.chat_message);
        welcome_settings.image_headline = image_headline.unwrap_or(welcome_settings.image_headline);
        welcome_settings.image_subtext = image_subline.unwrap_or(welcome_settings.image_subtext);
        welcome_settings.enabled = enabled.unwrap_or(welcome_settings.enabled);

        welcome_settings_mutation::update(db, welcome_settings).await?;
    } else {
        let welcome_settings = entity::welcome_settings::Model {
            id: 0,
            welcome_channel: 0,
            chat_message: chat_message
                .unwrap_or_else(|| "Hey {user}, welcome to **{guild_name}**".to_string()),
            image_headline: image_headline
                .unwrap_or_else(|| "{name} just joined the server".to_string()),
            image_subtext: image_subline
                .unwrap_or_else(|| "You are the #{members} member".to_string()),
            back_banner: 1,
            front_banner: 2,
            enabled: enabled.unwrap_or(false),
            create_user_id,
            create_date: Utc::now().naive_utc().to_string(),
            modify_date: None,
            modify_user_id: None,
        };

        let welcome_settings = welcome_settings_mutation::create(db, welcome_settings).await?;
        guild.welcome_settings_id = Some(welcome_settings.id);

        guild_mutation::update(db, &guild).await?;
    }

    Ok(guild)
}
