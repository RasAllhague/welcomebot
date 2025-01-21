use crate::{Context, PoiseError};
use poise::serenity_prelude::{self as serenity};
use welcome_service::guild_mutation;

/// Commands for moderating with the welcome bot
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

/// Settings of moderation features of the welcome bot
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
    #[description = "A role which should be automatic banned if a user has aquired this role"]
    autoban_role: Option<serenity::RoleId>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    // unwrap since we are in a guild only command
    let discord_guild = ctx.guild().unwrap().clone();
    let author_id = ctx.author().id.into();

    let mut guild =
        guild_mutation::get_or_create(db, discord_guild.id.into(), discord_guild.name, author_id)
            .await?;

    if let Some(role_id) = autoban_role {
        guild.auto_ban_role_id = Some(role_id.into());
        guild_mutation::update(db, &guild).await?;
    }
    if let Some(moderation_channel_id) = moderation_channel {
        guild.moderation_channel_id = Some(moderation_channel_id.id().into());
        guild_mutation::update(db, &guild).await?;
    }
    if let Some(ban_reason) = ban_reason {
        guild.ban_reason_template = Some(ban_reason);
        guild_mutation::update(db, &guild).await?;
    }

    Ok(())
}
