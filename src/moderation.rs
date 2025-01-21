use entity::{ban_entry, guild};
use log::{error, info, warn};
use poise::serenity_prelude::{self as serenity, ChannelId, CreateMessage, GuildId, User};
use welcome_service::{ban_entry_mutation, ban_entry_query::is_not_banned, guild_query};

use crate::{embed::BanEmbed, Data, PoiseError};

async fn ban_member_if_contains_autoban(
    ctx: &serenity::Context,
    guild: &guild::Model,
    member: &serenity::Member,
    event: &serenity::GuildMemberUpdateEvent,
) {
    let role_id = match guild.auto_ban_role_id {
        Some(role_id) => role_id,
        None => return,
    };

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
            }
            Err(why) => {
                error!(
                    "Could not ban: Id:'{}', name:'{}', because: {}",
                    member.user.id,
                    member.display_name(),
                    why
                );
            }
        }
    }
}

pub async fn ban_bot_user(
    ctx: &serenity::Context,
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
        if !is_not_banned(db, guild.id, member.user.id.into()).await? {
            return Ok(());
        }

        ban_member_if_contains_autoban(ctx, &guild, member, event).await;
    }
    Ok(())
}

pub async fn update_ban_log(
    ctx: &serenity::Context,
    data: &Data,
    guild_id: &GuildId,
    banned_user: &User,
    banned_by: i64,
) -> Result<(), PoiseError> {
    let db = &data.conn;

    let guild = match guild_query::get_by_guild_id(db, guild_id.get() as i64).await? {
        Some(g) => g,
        None => return Ok(()),
    };

    if let Some(ban) = guild_id
        .bans(ctx, None, None)
        .await?
        .iter()
        .find(|x| x.user.id == banned_user.id)
    {
        let ban_entry = ban_entry::Model {
            id: 0,
            guild_id: guild.id,
            user_id: banned_user.id.into(),
            user_name: banned_user.name.clone(),
            reason: ban.reason.clone(),
            create_user_id: banned_by,
            create_date: chrono::Utc::now(),
        };

        ban_entry_mutation::create(db, ban_entry).await?;

        if let Some(moderation_channel_id) = guild.moderation_channel_id {
            let moderation_channel = ChannelId::new(moderation_channel_id as u64);

            let embed = BanEmbed::new(
                banned_user.id.into(),
                banned_user.name.clone(),
                banned_user
                    .avatar_url()
                    .unwrap_or(banned_user.default_avatar_url()),
                ban.reason.clone(),
                "welcomebot".to_string(),
            )
            .to_embed();

            moderation_channel
                .send_message(&ctx.http, CreateMessage::new().embed(embed))
                .await?;

            info!("Sent message to moderation channel.");
        }
    }

    Ok(())
}
