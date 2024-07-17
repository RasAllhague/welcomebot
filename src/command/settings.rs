use crate::Data;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

#[poise::command(slash_command)]
pub async fn settings(ctx: Context<'_>) -> Result<(), PoiseError> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn welcome_message(ctx: Context<'_>) -> Result<(), PoiseError> {
    Ok(())
}
