use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use log::{debug, error, warn};
use poise::serenity_prelude::{
    self as serenity, AuditLogEntry, ChannelId, CreateMessage, GuildId,
    Timestamp, User, UserId, futures::lock::Mutex,
};
use uuid::Uuid;
use welcome_service::{ban_entry, guild};

use crate::{
    Data, PoiseError,
    embed::{BanEmbed, KickLogEmbed, SuspiciousUserEmbed, ToEmbed},
    interaction::{
        ButtonOnceEmbed, InteractionButton,
        button::{BanButton, IgnoreButton, KickButton, UnbanButton},
    },
    util::is_banned,
};

const MEMBER_KICK: u8 = 20;

/// Handles a suspicious user detected in the guild.
///
/// This function checks if the user is banned or has acquired an auto-ban role.
/// If neither condition is met, it sends an embed to the moderation channel
/// for further action.
///
/// # Arguments
/// * `ctx` - The Serenity context.
/// * `data` - The shared bot data.
/// * `new` - The updated member information.
/// * `event` - The guild member update event.
///
/// # Errors
/// Returns a [`PoiseError`] if any operation fails.
#[fastrace::trace]
pub async fn handle_suspicious_user(
    ctx: &serenity::Context,
    data: &Data,
    new: Option<&serenity::Member>,
    event: &serenity::GuildMemberUpdateEvent,
) -> Result<(), PoiseError> {
    let db = &data.conn;
    let guild_id: i64 = event.guild_id.into();

    let Some(guild) = guild::get_by_guild_id(db, guild_id).await? else {
        return Ok(());
    };

    if let Some(member) = new {
        if is_banned(ctx, &event.guild_id, member).await? {
            return Ok(());
        }

        if !punish_autoban_role(ctx, &guild, member, event).await {
            send_suspicious_user_embed(ctx, member, &guild).await?;
        }
    }
    Ok(())
}

/// Bans a user if they acquire an auto-ban role.
///
/// This function checks if the user has acquired a role that triggers an automatic ban.
/// If so, it bans the user and logs the action.
///
/// # Arguments
/// * `ctx` - The Serenity context.
/// * `guild` - The guild model.
/// * `member` - The member to check.
/// * `event` - The guild member update event.
///
/// # Returns
/// `true` if the user was banned, `false` otherwise.
#[fastrace::trace]
async fn punish_autoban_role(
    ctx: &serenity::Context,
    guild: &entity::guild::Model,
    member: &serenity::Member,
    event: &serenity::GuildMemberUpdateEvent,
) -> bool {
    let Some(role_id) = guild.auto_ban_role_id else {
        return false;
    };

    if event.roles.iter().any(|x| x.get() as i64 == role_id) {
        let ban_reason = guild
            .ban_reason_template
            .clone()
            .unwrap_or_else(|| "Banned due to choosing auto ban role.".to_string());

        if guild.punish_mode == "kick" {
            match member.kick_with_reason(&ctx, &ban_reason).await {
                Ok(()) => {
                    warn!(
                        "User kicked: Id:'{}', name:'{}'.",
                        member.user.id,
                        member.display_name()
                    );

                    return true;
                }
                Err(why) => {
                    error!(
                        "Could not kick: Id:'{}', name:'{}', because: {}",
                        member.user.id,
                        member.display_name(),
                        why
                    );

                    return false;
                }
            }
        } else {
            match member.ban_with_reason(&ctx, 7, ban_reason).await {
                Ok(()) => {
                    warn!(
                        "User banned: Id:'{}', name:'{}'.",
                        member.user.id,
                        member.display_name()
                    );

                    return true;
                }
                Err(why) => {
                    error!(
                        "Could not ban: Id:'{}', name:'{}', because: {}",
                        member.user.id,
                        member.display_name(),
                        why
                    );

                    return false;
                }
            }
        }
    }

    false
}

/// Updates the ban log for a banned user.
///
/// This function logs the ban in the database and sends a ban embed to the moderation channel.
///
/// # Arguments
/// * `ctx` - The Serenity context.
/// * `data` - The shared bot data.
/// * `guild_id` - The ID of the guild where the ban occurred.
/// * `banned_user` - The user who was banned.
/// * `banned_by` - The ID of the user or bot that issued the ban.
///
/// # Errors
/// Returns a [`PoiseError`] if any operation fails.
#[fastrace::trace]
pub async fn update_ban_log(
    ctx: &serenity::Context,
    data: &Data,
    guild_id: &GuildId,
    banned_user: &User,
    banned_by: i64,
) -> Result<(), PoiseError> {
    let db = &data.conn;

    let Some(guild) = guild::get_by_guild_id(db, guild_id.get() as i64).await? else {
        return Ok(());
    };

    if let Some(ban) = guild_id
        .bans(ctx, None, None)
        .await?
        .iter()
        .find(|x| x.user.id == banned_user.id)
    {
        let ban_entry = entity::ban_entry::Model {
            id: 0,
            guild_id: guild.id,
            user_id: banned_user.id.into(),
            user_name: banned_user.name.clone(),
            reason: ban.reason.clone(),
            create_user_id: banned_by,
            create_date: chrono::Utc::now(),
        };

        ban_entry::create(db, ban_entry).await?;

        if let Some(moderation_channel_id) = guild.moderation_channel_id {
            let moderation_channel = ChannelId::new(moderation_channel_id as u64);

            let embed = BanEmbed::new(
                banned_user.id.into(),
                banned_user.name.clone(),
                banned_user
                    .avatar_url()
                    .unwrap_or_else(|| banned_user.default_avatar_url()),
                ban.reason.clone(),
                "welcomebot".to_string(),
                None,
            );

            let mut interaction_embed = BanInteractionEmbed::new(embed);
            interaction_embed.send(ctx, &moderation_channel).await?;
        }
    }

    Ok(())
}

pub async fn send_audit_log_entry(
    ctx: &serenity::Context,
    data: &Data,
    guild_id: &GuildId,
    audit_log_entry: &AuditLogEntry,
) -> Result<(), PoiseError> {
    let db = &data.conn;

    debug!("Sending audit log entry");

    let Some(guild) = guild::get_by_guild_id(db, guild_id.get() as i64).await? else {
        debug!("No guild found for audit log entry");
        return Ok(());
    };

    match audit_log_entry.action.num() {
        MEMBER_KICK => {
            let create_user = audit_log_entry.user_id.to_user(ctx).await?;

            let Some(target_user_id) = audit_log_entry.target_id else {
                return Ok(());
            };

            let target_user_id: UserId = UserId::new(target_user_id.get());
            let target_user = target_user_id.to_user(ctx).await?;

            let kick_entry = entity::kick_entry::Model {
                id: 0,
                user_id: target_user_id.into(),
                user_name: target_user.name.clone(),
                reason: audit_log_entry.reason.clone(),
                guild_id: guild.id,
                create_user_id: create_user.id.into(),
                create_date: Utc::now(),
            };

            welcome_service::kick_entry::create(db, kick_entry).await?;

            let Some(moderation_channel_id) = guild.moderation_channel_id else {
                return Ok(());
            };

            let moderation_channel = ChannelId::new(moderation_channel_id as u64);

            let embed = KickLogEmbed::new(
                target_user.name.clone(),
                target_user.id.into(),
                create_user.name.clone(),
                create_user.id.into(),
                create_user
                    .avatar_url()
                    .unwrap_or(create_user.default_avatar_url()),
                audit_log_entry.reason.clone(),
                Timestamp::now(),
            )
            .to_embed();

            moderation_channel
                .send_message(ctx, CreateMessage::new().add_embed(embed))
                .await?;

            debug!("Send message to channel!");

            Ok(())
        }
        _ => Ok(()),
    }
}

/// Sends an embed for a suspicious user to the moderation channel.
///
/// This function creates and sends an embed for a user flagged as suspicious.
///
/// # Arguments
/// * `ctx` - The Serenity context.
/// * `member` - The suspicious member.
/// * `guild` - The guild model.
///
/// # Errors
/// Returns a [`PoiseError`] if sending the embed fails.
#[fastrace::trace]
pub async fn send_suspicious_user_embed(
    ctx: &serenity::Context,
    member: &serenity::Member,
    guild: &entity::guild::Model,
) -> Result<(), PoiseError> {
    if let (Some(timestamp), Some(moderation_channel_id)) = (
        member.unusual_dm_activity_until,
        guild.moderation_channel_id,
    ) {
        let moderation_channel = ChannelId::new(moderation_channel_id as u64);
        let suspicious_user_embed = SuspiciousUserEmbed::new(
            ctx.cache.current_user().name.clone(),
            member.user.id.into(),
            member.user.name.clone(),
            member
                .user
                .avatar_url()
                .unwrap_or_else(|| member.user.default_avatar_url()),
            timestamp,
        );

        let mut interaction_embed = SuspiciousUserInteractionEmbed::new(suspicious_user_embed);
        interaction_embed.send(ctx, &moderation_channel).await?;
    }

    Ok(())
}

/// Represents an interaction embed for banning a user.
#[derive(Clone)]
pub struct BanInteractionEmbed {
    /// The unique interaction ID.
    interaction_id: Uuid,
    /// The embed containing ban details.
    embed: BanEmbed,
    /// The buttons associated with the embed.
    buttons: Vec<Arc<Mutex<dyn InteractionButton<BanEmbed> + Send + Sync>>>,
}

impl BanInteractionEmbed {
    /// Creates a new `BanInteractionEmbed` instance.
    ///
    /// # Arguments
    /// * `embed` - The ban embed to associate with the interaction.
    ///
    /// # Returns
    /// A new `BanInteractionEmbed` instance.
    pub fn new(embed: BanEmbed) -> Self {
        let interaction_id = Uuid::new_v4();
        Self {
            interaction_id,
            embed,
            buttons: vec![Arc::new(Mutex::new(UnbanButton::new(interaction_id)))],
        }
    }
}

#[async_trait]
impl ButtonOnceEmbed<BanEmbed> for BanInteractionEmbed {
    fn interaction_id(&self) -> Uuid {
        self.interaction_id
    }

    fn embed(&self) -> BanEmbed {
        self.embed.clone()
    }

    fn buttons(&self) -> Vec<Arc<Mutex<dyn InteractionButton<BanEmbed> + Send + Sync>>> {
        self.buttons.clone()
    }
}

/// Represents an interaction embed for a suspicious user.
#[derive(Clone)]
pub struct SuspiciousUserInteractionEmbed {
    /// The unique interaction ID.
    interaction_id: Uuid,
    /// The embed containing suspicious user details.
    embed: SuspiciousUserEmbed,
    /// The buttons associated with the embed.
    buttons: Vec<Arc<Mutex<dyn InteractionButton<SuspiciousUserEmbed> + Send + Sync>>>,
}

impl SuspiciousUserInteractionEmbed {
    /// Creates a new `SuspiciousUserInteractionEmbed` instance.
    ///
    /// # Arguments
    /// * `embed` - The suspicious user embed to associate with the interaction.
    ///
    /// # Returns
    /// A new `SuspiciousUserInteractionEmbed` instance.
    pub fn new(embed: SuspiciousUserEmbed) -> Self {
        let interaction_id = Uuid::new_v4();
        Self {
            interaction_id,
            embed,
            buttons: vec![
                Arc::new(Mutex::new(BanButton::new(interaction_id))),
                Arc::new(Mutex::new(KickButton::new(interaction_id))),
                Arc::new(Mutex::new(IgnoreButton::new(interaction_id))),
            ],
        }
    }
}

#[async_trait]
impl ButtonOnceEmbed<SuspiciousUserEmbed> for SuspiciousUserInteractionEmbed {
    fn interaction_id(&self) -> Uuid {
        self.interaction_id
    }

    fn embed(&self) -> SuspiciousUserEmbed {
        self.embed.clone()
    }

    fn buttons(&self) -> Vec<Arc<Mutex<dyn InteractionButton<SuspiciousUserEmbed> + Send + Sync>>> {
        self.buttons.clone()
    }
}
