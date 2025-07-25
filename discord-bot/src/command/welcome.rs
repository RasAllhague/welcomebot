use chrono::Utc;
use migration::sea_orm::DbConn;
use poise::{
    CreateReply,
    serenity_prelude::{self as serenity},
};
use welcome_service::{guild, welcome_settings};

use crate::{Context, PoiseError};

/// Commands for welcoming users with the welcome bot.
///
/// This command serves as the entry point for welcome-related subcommands.
/// It is a slash command that is only available in guilds and requires the
/// user to have `ADMINISTRATOR` permissions.
///
/// # Errors
/// Returns a [`PoiseError`] if sending the response fails.
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

/// Settings for the welcome bot, allowing customization of its behavior.
///
/// This command allows administrators to configure the welcome bot's settings,
/// such as the welcome message, image headline, subline, and the channel where
/// welcome messages are sent.
///
/// # Arguments
/// * `ctx` - The command context.
/// * `chat_message` - An optional text for the chat welcome message. Placeholders: `{user}`, `{guild_name}`.
/// * `image_headline` - An optional text for the image headline. Placeholders: `{name}`.
/// * `image_subline` - An optional text for the image subline. Placeholders: `{members}`.
/// * `channel` - An optional text channel where welcome messages should be sent.
/// * `enabled` - An optional flag to enable or disable welcome messages.
///
/// # Errors
/// Returns a [`PoiseError`] if any database operation or response fails.
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
    #[description = "The text of the headline of the image. Placeholders: {name}"]
    image_headline: Option<String>,
    #[description = "The text of the subline of the image. Placeholders: {members}"]
    image_subline: Option<String>,
    #[description = "The channel where to send welcome messages to"]
    #[channel_types("Text")]
    channel: Option<serenity::Channel>,
    #[description = "Enables or disables the welcome message sending"] enabled: Option<bool>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    // Unwrap since this is a guild-only command
    let discord_guild = ctx.guild().unwrap().clone();
    let author_id = ctx.author().id.into();

    // Retrieve or create the guild entry in the database
    let guild =
        guild::get_or_create(db, discord_guild.id.into(), discord_guild.name, author_id).await?;
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

    // Send a confirmation message
    ctx.send(
        CreateReply::default()
            .content("Settings updated.")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

/// Updates the welcome settings for the guild.
///
/// This function updates the welcome settings in the database for the specified guild.
/// If no settings exist, it creates new ones.
///
/// # Arguments
/// * `db` - The database connection.
/// * `guild` - The guild model to update.
/// * `create_user_id` - The ID of the user making the changes.
/// * `chat_message` - An optional text for the chat welcome message.
/// * `image_headline` - An optional text for the image headline.
/// * `image_subline` - An optional text for the image subline.
/// * `enabled` - An optional flag to enable or disable welcome messages.
/// * `channel` - An optional channel ID where welcome messages should be sent.
///
/// # Errors
/// Returns a [`PoiseError`] if any database operation fails.
#[fastrace::trace]
async fn update_welcome_settings(
    db: &DbConn,
    mut guild: entity::guild::Model,
    create_user_id: i64,
    chat_message: Option<String>,
    image_headline: Option<String>,
    image_subline: Option<String>,
    enabled: Option<bool>,
    channel: Option<serenity::ChannelId>,
) -> Result<entity::guild::Model, PoiseError> {
    if let Some(mut welcome_settings) = welcome_settings::get_one(db, guild.id).await? {
        // Update existing welcome settings
        welcome_settings.welcome_channel = match channel {
            Some(c) => c.into(),
            None => welcome_settings.welcome_channel,
        };
        welcome_settings.chat_message = chat_message.unwrap_or(welcome_settings.chat_message);
        welcome_settings.image_headline = image_headline.unwrap_or(welcome_settings.image_headline);
        welcome_settings.image_subtext = image_subline.unwrap_or(welcome_settings.image_subtext);
        welcome_settings.enabled = enabled.unwrap_or(welcome_settings.enabled);

        welcome_settings::update(db, welcome_settings).await?;
    } else {
        // Create new welcome settings if none exist
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

        let welcome_settings = welcome_settings::create(db, welcome_settings).await?;
        guild.welcome_settings_id = Some(welcome_settings.id);

        guild::update(db, &guild).await?;
    }

    Ok(guild)
}
