use poise::serenity_prelude::{self as serenity};

use crate::PoiseError;

pub async fn is_banned(
    ctx: &serenity::Context,
    guild: &serenity::GuildId,
    member: &serenity::Member,
) -> Result<bool, PoiseError> {
    Ok(guild
        .bans(ctx, None, None)
        .await?
        .iter()
        .find(|x| x.user.id == member.user.id)
        .is_some())
}
