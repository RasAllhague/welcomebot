pub mod button;

use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use log::debug;
use poise::serenity_prelude::{
    ButtonStyle, ChannelId, ComponentInteraction, ComponentInteractionCollector, Context,
    CreateActionRow, CreateButton, CreateInteractionResponse, CreateMessage, EditMessage,
    futures::lock::Mutex,
};
use uuid::Uuid;

use crate::{PoiseError, embed::ToEmbed};

/// Trait representing an interaction button.
///
/// This trait defines the behavior of buttons that can be interacted with in Discord.
/// Buttons can have a name, style, label, and an associated action to execute.
#[async_trait]
pub trait InteractionButton<T: ToEmbed> {
    /// Returns the name of the button.
    ///
    /// The name is used to identify the button in interactions.
    fn name(&self) -> String;

    /// Returns the style of the button.
    ///
    /// The style determines the visual appearance of the button (e.g., primary, danger).
    fn style(&self) -> ButtonStyle;

    /// Returns the label of the button.
    ///
    /// The label is the text displayed on the button.
    fn label(&self) -> String;

    /// Converts the button to a `CreateButton` instance.
    ///
    /// # Arguments
    /// * `is_disabled` - Whether the button should be disabled.
    ///
    /// # Returns
    /// A `CreateButton` instance representing the button.
    fn to_create_button(&self, is_disabled: bool) -> CreateButton;

    /// Executes the button's action.
    ///
    /// # Arguments
    /// * `ctx` - The context in which the button is executed.
    /// * `interaction` - The component interaction that triggered the button.
    /// * `embed` - The embed associated with the button.
    ///
    /// # Errors
    /// Returns a [`PoiseError`] if the action fails.
    async fn execute(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
        embed: &T,
    ) -> Result<T, PoiseError>;

    /// Determines whether the button can be executed.
    ///
    /// # Arguments
    /// * `ctx` - The context in which the button is executed.
    /// * `interaction` - The component interaction that triggered the button.
    ///
    /// # Returns
    /// `true` if the button can be executed, `false` otherwise.
    fn can_execute(&self, _ctx: &Context, interaction: &ComponentInteraction) -> bool {
        interaction.data.custom_id == self.name()
    }
}

/// Trait representing an embed with buttons that can be sent once.
///
/// This trait defines the behavior of embeds that include interactive buttons.
/// The embed can be sent to a channel, and the buttons can be interacted with.
#[async_trait]
pub trait ButtonOnceEmbed<E: ToEmbed + std::marker::Send + Clone> {
    /// Returns the interaction ID associated with the embed.
    ///
    /// The interaction ID is used to uniquely identify the embed and its buttons.
    fn interaction_id(&self) -> Uuid;

    /// Gets the embed to send.
    ///
    /// # Returns
    /// The embed associated with the buttons.
    fn embed(&self) -> E;

    /// Gets the buttons to send.
    ///
    /// # Returns
    /// A vector of buttons associated with the embed.
    fn buttons(&self) -> Vec<Arc<Mutex<dyn InteractionButton<E> + Send + Sync>>>;

    /// Sends the embed to a channel.
    ///
    /// This method sends the embed along with its buttons to the specified channel.
    /// It listens for button interactions and updates the embed accordingly.
    ///
    /// # Arguments
    /// * `ctx` - The context to send the embed with.
    /// * `channel_id` - The channel to send the embed to.
    ///
    /// # Errors
    /// Returns a [`PoiseError`] if sending the embed or handling interactions fails.
    #[fastrace::trace]
    async fn send(&mut self, ctx: &Context, channel_id: &ChannelId) -> Result<(), PoiseError> {
        let mut embed = self.embed().clone();

        // Create the initial message with buttons
        let create_message = {
            let mut buttons = Vec::new();

            for button in self.buttons() {
                buttons.push(button.lock().await.to_create_button(false));
            }

            let components = CreateActionRow::Buttons(buttons);

            CreateMessage::default()
                .embed(embed.to_embed())
                .components(vec![components])
        };

        let mut message = channel_id.send_message(&ctx, create_message).await?;

        debug!("Sent message to {} channel.", channel_id.name(ctx).await?);

        let cloned_id = self.interaction_id().clone();

        // Wait for a button interaction
        if let Some(press) = ComponentInteractionCollector::new(ctx)
            .filter(move |press| press.data.custom_id.starts_with(&cloned_id.to_string()))
            .timeout(Duration::from_secs(86400)) // 24 hours timeout
            .await
        {
            // Acknowledge the interaction
            press
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await?;

            // Execute the action for the pressed button
            for button in self.buttons().iter() {
                if button.lock().await.can_execute(ctx, &press) {
                    embed = button.lock().await.execute(ctx, &press, &embed).await?;
                }
            }

            debug!(
                "Handled button press for {} in {} channel.",
                press.data.custom_id,
                channel_id.name(ctx).await?
            );
        }

        // Update the message to disable buttons after interaction
        let edit_message = {
            let mut buttons = Vec::new();

            for button in self.buttons() {
                buttons.push(button.lock().await.to_create_button(true));
            }

            let components = CreateActionRow::Buttons(buttons);

            EditMessage::default()
                .embed(embed.to_embed())
                .components(vec![components])
        };

        message.edit(ctx, edit_message).await?;

        Ok(())
    }
}
