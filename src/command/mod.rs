use crate::Data;

pub mod settings;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[poise::command(slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<(), PoiseError> {
    let response = format!("Current running version: {VERSION}");
    ctx.say(response).await?;
    Ok(())
}
