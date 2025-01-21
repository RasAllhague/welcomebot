use crate::{Context, PoiseError};

static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Displays the current version of the bot running
#[poise::command(slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<(), PoiseError> {
    let response = format!("Current running version: {VERSION}");
    ctx.say(response).await?;

    Ok(())
}
