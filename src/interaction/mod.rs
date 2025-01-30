pub mod button;

use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use log::debug;
use poise::serenity_prelude::{
    futures::lock::Mutex, ButtonStyle, ChannelId, ComponentInteraction,
    ComponentInteractionCollector, Context, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateMessage, EditMessage,
};
use uuid::Uuid;

use crate::{embed::{self, ToEmbed}, PoiseError};

#[async_trait]
pub trait InteractionButton {
    fn name(&self) -> String;
    fn style(&self) -> ButtonStyle;
    fn label(&self) -> String;
    fn to_create_button(&self, is_disabled: bool) -> CreateButton;
    async fn execute(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> Result<(), PoiseError>;
    fn can_execute(&self, ctx: &Context, interaction: &ComponentInteraction) -> bool;
}

#[async_trait]
pub trait ButtonOnceEmbed<E: ToEmbed> {
    fn interaction_id(&self) -> Uuid;
    /// Gets the embed to send.
    fn embed(&self) -> E;
    /// Gets the buttons to send.
    fn buttons(&self) -> Vec<Arc<Mutex<dyn InteractionButton + Send + Sync>>>;
    /// Send the embed to a channel.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The context to send the embed with.
    /// - `channel_id` - The channel to send the embed to.
    async fn send(&mut self, ctx: &Context, channel_id: &ChannelId) -> Result<(), PoiseError> {
        let create_message = {
            let mut buttons = Vec::new();

            for button in self.buttons() {
                buttons.push(button.lock().await.to_create_button(false));
            }

            let components = CreateActionRow::Buttons(buttons);

            CreateMessage::default()
                .embed(self.embed().to_embed())
                .components(vec![components])
        };

        let mut message = channel_id.send_message(&ctx, create_message).await?;

        debug!("Sent message to {} channel.", channel_id.name(ctx).await?);

        let cloned_id = self.interaction_id().clone();

        // Only hanlde one interaction filtered to the button id.
        if let Some(press) = ComponentInteractionCollector::new(ctx)
            .filter(move |press| press.data.custom_id.starts_with(&cloned_id.to_string()))
            .timeout(Duration::from_secs(86400))
            .await
        {
            press
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await?;

            for button in self.buttons().iter() {
                if button.lock().await.can_execute(ctx, &press) {
                    button.lock().await.execute(ctx, &press).await?;
                }
            }

            debug!(
                "Handled button press for {} in {} channel.",
                press.data.custom_id,
                channel_id.name(ctx).await?
            );
        }

        let edit_message = {
            let mut buttons = Vec::new();

            for button in self.buttons() {
                buttons.push(button.lock().await.to_create_button(true));
            }

            let components = CreateActionRow::Buttons(buttons);

            EditMessage::default()
                .embed(self.embed().to_embed())
                .components(vec![components])
        };

        message.edit(ctx, edit_message).await?;

        Ok(())
    }
}
