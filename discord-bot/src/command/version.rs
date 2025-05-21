use crate::{Context, PoiseError};

/// The current version of the bot, retrieved from the `CARGO_PKG_VERSION` environment variable.
static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Displays the current version of the bot.
///
/// This command provides the version of the bot that is currently running. It is a simple
/// slash command that can be used to verify the deployed version of the bot.
///
/// # Arguments
/// * `ctx` - The command context.
///
/// # Errors
/// Returns a [`PoiseError`] if sending the response fails.
#[fastrace::trace]
#[poise::command(slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<(), PoiseError> {
    let response = format!("Current running version: {VERSION}");
    ctx.say(response).await?;

    Ok(())
}
