use std::sync::Arc;

use async_trait::async_trait;
use entity::{ban_entry, guild};
use log::{error, warn};
use poise::serenity_prelude::{
    self as serenity, futures::lock::Mutex, ChannelId, CreateMessage, GuildId, User,
};
use uuid::Uuid;
use welcome_service::{ban_entry_mutation, guild_query};

use crate::{
    embed::{BanEmbed, KickEmbed, SuspiciousUserEmbed, ToEmbed},
    interaction::{
        button::{BanButton, IgnoreButton, KickButton, UnbanButton},
        ButtonOnceEmbed, InteractionButton,
    },
    util::is_banned,
    Data, PoiseError,
};

pub async fn handle_suspicious_user(
    ctx: &serenity::Context,
    data: &Data,
    new: Option<&serenity::Member>,
    event: &serenity::GuildMemberUpdateEvent,
) -> Result<(), PoiseError> {
    let db = &data.conn;
    let guild_id: i64 = event.guild_id.into();

    let Some(guild) = guild_query::get_by_guild_id(db, guild_id).await? else {
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

async fn punish_autoban_role(
    ctx: &serenity::Context,
    guild: &guild::Model,
    member: &serenity::Member,
    event: &serenity::GuildMemberUpdateEvent,
) -> bool {
    let Some(role_id) = guild.auto_ban_role_id else {
        return false;
    };

    if !event.roles.iter().any(|x| x.get() as i64 == role_id) {
        return false;
    }

    let ban_reason = guild
        .ban_reason_template
        .clone()
        .unwrap_or_else(|| "Banned due to choosing auto ban role.".to_string());

    match guild.punish_mode.as_str() {
        "kick" => {
            if kick_with_logging(ctx, member, ban_reason.clone()).await {
                if let Some(moderation_channel_id) = guild.moderation_channel_id {
                    let embed = KickEmbed::new(
                        member.user.id.into(),
                        member.display_name().to_string(),
                        member
                            .avatar_url()
                            .unwrap_or(member.user.default_avatar_url()),
                        Some(ban_reason),
                        ctx.cache.current_user().name.clone(),
                    );

                    let _ = ChannelId::new(moderation_channel_id as u64)
                        .send_message(&ctx, CreateMessage::default().embed(embed.to_embed()))
                        .await;
                }

                return true;
            } else {
                return false;
            }
        }
        "ban" => ban_with_logging(ctx, member, ban_reason).await,
        _ => false,
    }
}

async fn ban_with_logging(
    ctx: &serenity::Context,
    member: &serenity::Member,
    ban_reason: String,
) -> bool {
    match member.ban_with_reason(&ctx, 7, ban_reason).await {
        Ok(()) => {
            warn!(
                "User banned: Id:'{}', name:'{}'.",
                member.user.id,
                member.display_name()
            );

            true
        }
        Err(why) => {
            error!(
                "Could not ban: Id:'{}', name:'{}', because: {}",
                member.user.id,
                member.display_name(),
                why
            );

            false
        }
    }
}

async fn kick_with_logging(
    ctx: &serenity::Context,
    member: &serenity::Member,
    kick_reason: String,
) -> bool {
    match member.kick_with_reason(&ctx, &kick_reason).await {
        Ok(()) => {
            warn!(
                "User kicked: Id:'{}', name:'{}'.",
                member.user.id,
                member.display_name()
            );

            true
        }
        Err(why) => {
            error!(
                "Could not kick: Id:'{}', name:'{}', because: {}",
                member.user.id,
                member.display_name(),
                why
            );

            false
        }
    }
}

pub async fn update_ban_log(
    ctx: &serenity::Context,
    data: &Data,
    guild_id: &GuildId,
    banned_user: &User,
    banned_by: i64,
) -> Result<(), PoiseError> {
    let db = &data.conn;

    let Some(guild) = guild_query::get_by_guild_id(db, guild_id.get() as i64).await? else {
        return Ok(());
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

#[derive(Clone)]
pub struct BanInteractionEmbed {
    interaction_id: Uuid,
    embed: BanEmbed,
    buttons: Vec<Arc<Mutex<dyn InteractionButton<BanEmbed> + Send + Sync>>>,
}

impl BanInteractionEmbed {
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

#[derive(Clone)]
pub struct SuspiciousUserInteractionEmbed {
    interaction_id: Uuid,
    embed: SuspiciousUserEmbed,
    buttons: Vec<Arc<Mutex<dyn InteractionButton<SuspiciousUserEmbed> + Send + Sync>>>,
}

impl SuspiciousUserInteractionEmbed {
    pub fn new(embed: SuspiciousUserEmbed) -> Self {
        let interaction_id = Uuid::new_v4();
        Self {
            interaction_id: Uuid::new_v4(),
            embed,
            buttons: vec![
                Arc::new(Mutex::new(BanButton::new(interaction_id))),
                Arc::new(Mutex::new(KickButton::new(interaction_id))),
                Arc::new(Mutex::new(IgnoreButton::new(interaction_id))),
            ],
        }
    }
}

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
