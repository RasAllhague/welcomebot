use poise::serenity_prelude::{self as serenity};

use crate::PoiseError;

/// Checks if a member is banned in a guild.
///
/// This function retrieves the list of bans for the specified guild and checks
/// if the given member is present in the list of banned users.
///
/// # Arguments
/// * `ctx` - The Serenity context.
/// * `guild` - The ID of the guild to check for bans.
/// * `member` - The member to check if they are banned.
///
/// # Returns
/// Returns `Ok(true)` if the member is banned, `Ok(false)` otherwise.
/// Returns a [`PoiseError`] if retrieving the list of bans fails.
///
/// # Errors
/// This function will return an error if the bot lacks the necessary permissions
/// to retrieve the list of bans or if there is an issue with the Discord API.
#[fastrace::trace]
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
