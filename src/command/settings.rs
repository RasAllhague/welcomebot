use poise::serenity_prelude::{self as serenity, CreateMessage};

use crate::{error::Error, Data};

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// Settings of welcome bot. With this you can update its behaviour.
#[poise::command(slash_command, guild_only)]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "The text of the welcome message"] message: Option<String>,
    #[description = "The channel where to send welcome messages to"] channel: Option<
        serenity::Channel,
    >,
) -> Result<(), PoiseError> {
    if let Some(channel) = channel {
        if let Err(why) = update_welcome_channel(&ctx, channel).await {
            ctx.channel_id()
                .send_message(
                    &ctx,
                    CreateMessage::new()
                        .content(format!("Could not update welcome channel: {why}")),
                )
                .await?;
        }
    }
    if let Some(message) = message {
        if let Err(why) = update_welcome_message(&ctx, &message).await {
            ctx.channel_id()
                .send_message(
                    &ctx,
                    CreateMessage::new()
                        .content(format!("Could not update welcome message: {why}")),
                )
                .await?;
        }
    }

    ctx.say("Finished updating.").await?;

    Ok(())
}

async fn update_welcome_channel(
    ctx: &Context<'_>,
    channel: serenity::Channel,
) -> Result<(), PoiseError> {
    match channel {
        serenity::Channel::Guild(guild_channel) => match guild_channel.kind {
            serenity::ChannelType::Text => Ok(()),
            _ => Err(Box::new(Error::InvalidTextChannel(guild_channel.name))),
        },
        _ => {
            let name = channel.id().name(&ctx).await?;
            Err(Box::new(Error::InvalidTextChannel(name)))
        }
    }
}

async fn update_welcome_message(
    ctx: &Context<'_>,
    welcome_message: &str,
) -> Result<(), PoiseError> {
    Err(Box::new(Error::InvalidTextChannel(String::from(
        welcome_message,
    ))))
}
