use entity::{ban_entry, guild};
use log::{error, info, warn};
use poise::serenity_prelude::{self as serenity, ChannelId, CreateMessage};
use welcome_service::{ban_entry_mutation, ban_entry_query::is_not_banned, guild_query};

use crate::{Data, PoiseError};

pub async fn ban_member_if_contains_autoban(
    ctx: &serenity::Context,
    guild: &guild::Model,
    member: &serenity::Member,
    event: &serenity::GuildMemberUpdateEvent,
) -> Result<bool, PoiseError> {
    if let Some(role_id) = guild.auto_ban_role_id {
        if event.roles.iter().any(|x| x.get() as i64 == role_id) {
            let ban_reason = guild
                .ban_reason_template
                .clone()
                .unwrap_or("Banned due to choosing auto ban role.".to_string());

            match member.ban_with_reason(&ctx, 7, ban_reason).await {
                Ok(_) => {
                    warn!(
                        "User banned: Id:'{}', name:'{}'.",
                        member.user.id,
                        member.display_name()
                    );

                    return Ok(true);
                }
                Err(why) => {
                    error!(
                        "Could not ban: Id:'{}', name:'{}', because: {}",
                        member.user.id,
                        member.display_name(),
                        why
                    );

                    return Ok(false);
                }
            }
        }
    }
    Ok(false)
}

pub async fn ban_bot_user(
    ctx: &serenity::Context,
    framework: poise::FrameworkContext<'_, Data, PoiseError>,
    data: &Data,
    new: &Option<serenity::Member>,
    event: &serenity::GuildMemberUpdateEvent,
) -> Result<(), PoiseError> {
    let db = &data.conn;
    let guild_id: i64 = event.guild_id.into();

    let guild = match guild_query::get_by_guild_id(db, guild_id).await? {
        Some(g) => g,
        None => return Ok(()),
    };

    if let Some(member) = new {
        if !is_not_banned(db, guild.id, member.user.id.into()).await?
        {
            return Ok(());
        }
        if !ban_member_if_contains_autoban(ctx, &guild, member, event).await?
        {
            return Ok(());
        }

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
    Ok(())
}