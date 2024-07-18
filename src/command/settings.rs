use poise::serenity_prelude;

use crate::Data;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// Settings of welcome bot. With this you can update its behaviour.
#[poise::command(slash_command, guild_only)]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "The text of the welcome message"] message: Option<String>,
    #[description = "The channel where to send welcome messages to"] channel: Option<serenity_prelude::Channel>,
) -> Result<(), PoiseError> {
    ctx.say("This is a test.").await?;

    Ok(())
}