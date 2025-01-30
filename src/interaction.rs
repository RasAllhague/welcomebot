use std::time::Duration;

use async_trait::async_trait;
use log::{debug, info};
use poise::serenity_prelude::{
    ButtonStyle, ChannelId, ComponentInteraction, ComponentInteractionCollector, Context,
    CreateActionRow, CreateButton, CreateInteractionResponse, CreateMessage, EditMessage,
};

use crate::{
    embed::{BanEmbed, SuspiciousUserEmbed, ToEmbed},
    PoiseError,
};

#[async_trait]
pub trait InteractionButton {
    fn name(&self) -> String;
    fn style(&self) -> ButtonStyle;
    fn label(&self) -> String;
    fn to_create_button(&self, is_disabled: bool) -> CreateButton;
    async fn execute(&self, ctx: &Context, interaction: &ComponentInteraction) -> Result<(), PoiseError>;
    fn can_execute(&self, ctx: &Context, interaction: &ComponentInteraction) -> bool;
}

#[derive(Clone, Debug)]
pub struct Button {
    pub name: String,
    pub style: ButtonStyle,
    pub label: String,
}

impl Button {
    pub fn new(name: String, style: ButtonStyle, label: String) -> Self {
        Self { name, style, label }
    }

    pub fn to_create_button(&self, is_disabled: bool) -> CreateButton {
        CreateButton::new(&self.name)
            .style(self.style)
            .label(&self.label)
            .disabled(is_disabled)
    }
}

#[async_trait]
impl InteractionButton for Button {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn style(&self) -> ButtonStyle {
        self.style
    }

    fn label(&self) -> String {
        self.label.clone()
    }

    fn to_create_button(&self, is_disabled: bool) -> CreateButton {
        CreateButton::new(&self.name)
            .style(self.style)
            .label(&self.label)
            .disabled(is_disabled)
    }

    async fn execute(&self, ctx: &Context, interaction: &ComponentInteraction) -> Result<(), PoiseError> {
        debug!("Button {} pressed by {}.", self.name, interaction.user.name);

        Ok(())
    }

    fn can_execute(&self, _ctx: &Context, _interaction: &ComponentInteraction) -> bool {
        true
    }
}

#[async_trait]
pub trait ButtonOnceEmbed<E: ToEmbed> {
    /// Gets the embed to send.
    fn embed(&self) -> E;
    /// Gets the buttons to send.
    fn buttons(&self) -> Vec<Button>;
    /// Handles the button press.
    async fn handle_button_press(
        &mut self,
        ctx: &Context,
        press: &ComponentInteraction,
    ) -> Result<(), PoiseError>;
    /// Send the embed to a channel.
    ///
    /// # Arguments
    ///
    /// - `ctx` - The context to send the embed with.
    /// - `channel_id` - The channel to send the embed to.
    async fn send(&mut self, ctx: &Context, channel_id: &ChannelId) -> Result<(), PoiseError> {
        let button_id = uuid::Uuid::new_v4();

        let buttons = self
            .buttons()
            .iter()
            .map(|button| {
                Button::new(
                    format!("{}{}", button_id, button.name),
                    button.style,
                    button.label.clone(),
                )
            })
            .collect::<Vec<_>>();

        let create_message = {
            let buttons = buttons
                .iter()
                .map(|button| button.to_create_button(false))
                .collect::<Vec<_>>();

            let components = CreateActionRow::Buttons(buttons);

            CreateMessage::default()
                .embed(self.embed().to_embed())
                .components(vec![components])
        };

        let mut message = channel_id.send_message(&ctx, create_message).await?;

        debug!("Sent message to {} channel.", channel_id.name(ctx).await?);

        // Only hanlde one interaction filtered to the button id.
        if let Some(press) = ComponentInteractionCollector::new(ctx)
            .filter(move |press| press.data.custom_id.starts_with(&button_id.to_string()))
            .timeout(Duration::from_secs(86400))
            .await
        {
            press
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await?;

            self.handle_button_press(ctx, &press).await?;

            debug!(
                "Handled button press for {} in {} channel.",
                press.data.custom_id,
                channel_id.name(ctx).await?
            );
        }

        let edit_message = {
            let buttons = buttons
                .iter()
                .map(|button| button.to_create_button(true))
                .collect::<Vec<_>>();

            let components = CreateActionRow::Buttons(buttons);

            EditMessage::default()
                .embed(self.embed().to_embed())
                .components(vec![components])
        };

        message.edit(ctx, edit_message).await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct BanInteractionEmbed {
    embed: BanEmbed,
    buttons: Vec<Button>,
}

impl BanInteractionEmbed {
    pub fn new(embed: BanEmbed) -> Self {
        Self {
            embed,
            buttons: vec![Button::new(
                "Unban".to_string(),
                ButtonStyle::Primary,
                "Unban".to_string(),
            )],
        }
    }
}

#[async_trait]
impl ButtonOnceEmbed<BanEmbed> for BanInteractionEmbed {
    fn embed(&self) -> BanEmbed {
        self.embed.clone()
    }

    fn buttons(&self) -> Vec<Button> {
        self.buttons.clone()
    }

    async fn handle_button_press(
        &mut self,
        ctx: &Context,
        press: &ComponentInteraction,
    ) -> Result<(), PoiseError> {
        if let Some(guild_id) = press.guild_id {
            let user_id = self.embed.user_id as u64;

            guild_id.unban(ctx, user_id).await?;

            guild_id.unban(ctx, self.embed.user_id as u64).await?;
            self.embed.unbanned_by = Some(press.user.name.clone());

            info!(
                "Unbanned {}/{} from guild {} by {}/{}",
                self.embed.user_name, self.embed.user_id, guild_id, press.user.name, press.user.id
            );
        }

        Ok(())
    }
}

// #[derive(Clone, Debug)]
// pub struct SuspiciousUserInteractionEmbed {
//     embed: SuspiciousUserEmbed,
//     buttons: Vec<Button>,
// }

// impl SuspiciousUserInteractionEmbed {
//     pub fn new(embed: SuspiciousUserEmbed) -> Self {
//         Self {
//             embed,
//             buttons: vec![
//                 Button::new("Ban".to_string(), ButtonStyle::Danger, "Ban".to_string()),
//                 Button::new(
//                     "Kick".to_string(),
//                     ButtonStyle::Secondary,
//                     "Kick".to_string(),
//                 ),
//             ],
//         }
//     }
// }

// impl ButtonOnceEmbed<SuspiciousUserEmbed> for SuspiciousUserInteractionEmbed {
//     fn embed(&self) -> SuspiciousUserEmbed {
//         self.embed.clone()
//     }

//     fn buttons(&self) -> Vec<Button> {
//         self.buttons.clone()
//     }

//     async fn handle_button_press(
//         &mut self,
//         ctx: &Context,
//         press: &ComponentInteraction,
//     ) -> Result<(), PoiseError> {
//         if let Some(guild_id) = press.guild_id {
            

//             info!(
//                 "Unbanned {}/{} from guild {} by {}/{}",
//                 self.embed.user_name, self.embed.user_id, guild_id, press.user.name, press.user.id
//             );
//         }

//         Ok(())
//     }
// }
