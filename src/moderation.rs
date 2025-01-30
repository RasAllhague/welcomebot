use entity::{ban_entry, guild};
use log::{error, info, warn};
use poise::serenity_prelude::{
    self as serenity, ChannelId, CreateButton, CreateMessage, EditMessage, GuildId, User,
};
use welcome_service::{ban_entry_mutation, guild_query};

use crate::{
    embed::{BanEmbed, ToEmbed},
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

            handle_ban_button(ctx, guild_id, &moderation_channel, embed).await?;
        }
    }

    Ok(())
}

async fn handle_ban_button(
    ctx: &serenity::Context,
    guild_id: &GuildId,
    channel_id: &ChannelId,
    mut ban_embed: BanEmbed,
) -> Result<(), PoiseError> {
    let button_id = uuid::Uuid::new_v4();
    let unban_button = format!("{button_id}unban");

    let create_message = {
        let components =
            serenity::CreateActionRow::Buttons(vec![create_unban_button(&unban_button, false)]);

        CreateMessage::default()
            .embed(ban_embed.to_embed())
            .components(vec![components])
    };

    let mut message = channel_id.send_message(&ctx.http, create_message).await?;

    info!("Sent message to moderation channel.");

    let cloned_button_id = unban_button.clone();

    if let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id == cloned_button_id)
        .timeout(std::time::Duration::from_secs(86400))
        .await
    {
        guild_id.unban(ctx, ban_embed.user_id as u64).await?;
        ban_embed.unbanned_by = Some(press.user.name.clone());

        press
            .create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
            .await?;

        info!(
            "Unbanned {}/{} from guild {} by {}/{}",
            ban_embed.user_name, ban_embed.user_id, guild_id, press.user.name, press.user.id
        );
    }

    let edit_message = {
        let components =
            serenity::CreateActionRow::Buttons(vec![create_unban_button(&unban_button, true)]);

        EditMessage::default()
            .embed(ban_embed.to_embed())
            .components(vec![components])
    };

    message.edit(ctx, edit_message).await?;

    Ok(())
}

fn create_unban_button(button_id: &str, disabled: bool) -> CreateButton {
    CreateButton::new(button_id)
        .style(serenity::ButtonStyle::Primary)
        .label("Unban")
        .disabled(disabled)
}
