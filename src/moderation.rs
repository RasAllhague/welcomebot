use std::sync::Arc;

use async_trait::async_trait;
use entity::{ban_entry, guild};
use log::{error, warn};
use poise::serenity_prelude::{
    self as serenity, futures::lock::Mutex, ChannelId,
    CreateButton, GuildId, User,
};
use uuid::Uuid;
use welcome_service::{ban_entry_mutation, guild_query};

use crate::{
    embed::BanEmbed,
    interaction::{button::UnbanButton, ButtonOnceEmbed, InteractionButton},
    util::is_banned,
    Data, PoiseError,
};

pub async fn ban_suspicious_user(
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

        ban_autoban_role(ctx, &guild, member, event).await;
    }
    Ok(())
}

async fn ban_autoban_role(
    ctx: &serenity::Context,
    guild: &guild::Model,
    member: &serenity::Member,
    event: &serenity::GuildMemberUpdateEvent,
) {
    let Some(role_id) = guild.auto_ban_role_id else {
        return;
    };

    if event.roles.iter().any(|x| x.get() as i64 == role_id) {
        let ban_reason = guild
            .ban_reason_template
            .clone()
            .unwrap_or_else(|| "Banned due to choosing auto ban role.".to_string());

        match member.ban_with_reason(&ctx, 7, ban_reason).await {
            Ok(()) => {
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

fn create_unban_button(button_id: &str, disabled: bool) -> CreateButton {
    CreateButton::new(button_id)
        .style(serenity::ButtonStyle::Primary)
        .label("Unban")
        .disabled(disabled)
}

#[derive(Clone)]
pub struct BanInteractionEmbed {
    interaction_id: Uuid,
    embed: BanEmbed,
    buttons: Vec<Arc<Mutex<dyn InteractionButton + Send + Sync>>>,
}

impl BanInteractionEmbed {
    pub fn new(embed: BanEmbed) -> Self {
        Self {
            interaction_id: Uuid::new_v4(),
            embed,
            buttons: vec![Arc::new(Mutex::new(UnbanButton::new()))],
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

    fn buttons(&self) -> Vec<Arc<Mutex<dyn InteractionButton + Send + Sync>>> {
        self.buttons.clone()
    }
}
