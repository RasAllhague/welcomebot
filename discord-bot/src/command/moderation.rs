use crate::{Context, PoiseError};
use poise::{
    CreateReply,
    serenity_prelude::{self as serenity},
};
use welcome_service::guild::{get_or_create, update};

/// Commands for moderating with the welcome bot.
///
/// This command serves as the entry point for moderation-related subcommands.
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
pub async fn moderation(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum PunishMode {
    #[name = "Kick"]
    Kick,
    #[name = "Ban"]
    Ban,
}

impl ToString for PunishMode {
    fn to_string(&self) -> String {
        match self {
            PunishMode::Kick => "kick".to_owned(),
            PunishMode::Ban => "ban".to_owned(),
        }
    }
}

/// Settings of moderation features of the welcome bot.
///
/// This command allows administrators to configure moderation-related settings
/// for the welcome bot, such as setting a moderation log channel, defining a
/// ban reason template, or specifying a role that should trigger automatic bans.
///
/// # Arguments
/// * `ctx` - The command context.
/// * `moderation_channel` - An optional text channel where moderation logs should be sent.
/// * `ban_reason` - An optional template for the ban message.
/// * `autoban_role` - An optional role that triggers automatic bans when assigned to a user.
///
/// # Errors
/// Returns a [`PoiseError`] if any database operation or response fails.
#[fastrace::trace]
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "A channel where moderation logs should be sent to"]
    #[channel_types("Text")]
    moderation_channel: Option<serenity::Channel>,
    #[description = "The text of the ban message."] ban_reason: Option<String>,
    #[description = "A role which should be automatically banned if a user has acquired this role"]
    autoban_role: Option<serenity::RoleId>,
    #[description = "Should it kick or should it ban?"]
    punish_mode: Option<PunishMode>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    // Unwrap since this is a guild-only command
    let discord_guild = ctx.guild().unwrap().clone();
    let author_id = ctx.author().id.into();

    // Retrieve or create the guild entry in the database
    let mut guild =
        get_or_create(db, discord_guild.id.into(), discord_guild.name, author_id).await?;

    // Update the auto-ban role if provided
    if let Some(role_id) = autoban_role {
        guild.auto_ban_role_id = Some(role_id.into());
        update(db, &guild).await?;
    }

    // Update the moderation channel if provided
    if let Some(moderation_channel_id) = moderation_channel {
        guild.moderation_channel_id = Some(moderation_channel_id.id().into());
        update(db, &guild).await?;
    }

    // Update the ban reason template if provided
    if let Some(ban_reason) = ban_reason {
        guild.ban_reason_template = Some(ban_reason);
        update(db, &guild).await?;
    }

        // Update the ban reason template if provided
    if let Some(punish_mode) = punish_mode {
        guild.punish_mode = punish_mode.to_string();
        update(db, &guild).await?;
    }

    // Send a confirmation message
    ctx.send(
        CreateReply::default()
            .content("Settings updated.")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
