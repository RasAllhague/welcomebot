use entity::guild;
use log::{error, warn};
use poise::serenity_prelude::{self as serenity, ChannelId, CreateAttachment, CreateMessage};

use crate::PoiseError;

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
