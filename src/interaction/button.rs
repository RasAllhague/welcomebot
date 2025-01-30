use async_trait::async_trait;
use log::debug;
use poise::serenity_prelude::{ButtonStyle, ComponentInteraction, Context, CreateButton};

use crate::{interaction::InteractionButton, PoiseError};

#[derive(Clone, Debug)]
pub struct KickButton {
    pub name: String,
    pub style: ButtonStyle,
    pub label: String,
}

impl KickButton {
    pub fn new() -> Self {
        Self { name: "Kick".to_string(), style: ButtonStyle::Secondary, label: "Kick".to_string() }
    }
}

#[async_trait]
impl InteractionButton for KickButton {
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

    async fn execute(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> Result<(), PoiseError> {
        debug!(
            "Kick Button {} pressed by {}.",
            self.name, interaction.user.name
        );

        Ok(())
    }

    fn can_execute(&self, _ctx: &Context, _interaction: &ComponentInteraction) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct BanButton {
    pub name: String,
    pub style: ButtonStyle,
    pub label: String,
}

impl BanButton {
    pub fn new() -> Self {
        Self { name: "Ban".to_string(), style: ButtonStyle::Danger, label: "Ban".to_string() }
    }
}

#[async_trait]
impl InteractionButton for BanButton {
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

    async fn execute(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> Result<(), PoiseError> {
        debug!(
            "Ban Button {} pressed by {}.",
            self.name, interaction.user.name
        );

        Ok(())
    }

    fn can_execute(&self, _ctx: &Context, _interaction: &ComponentInteraction) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct UnbanButton {
    pub name: String,
    pub style: ButtonStyle,
    pub label: String,
}

impl UnbanButton {
    pub fn new() -> Self {
        Self { name: "Unban".to_string(), style: ButtonStyle::Primary, label: "Unban".to_string() }
    }
}

#[async_trait]
impl InteractionButton for UnbanButton {
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

    async fn execute(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> Result<(), PoiseError> {
        if let Some(guild_id) = interaction.guild_id {
            // guild_id.unban(ctx, ban_embed.user_id as u64).await?;
                
            // interaction
            //     .create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
            //     .await?;
    
            // info!(
            //     "Unbanned {}/{} from guild {} by {}/{}",
            //     ban_embed.user_name, ban_embed.user_id, guild_id, press.user.name, press.user.id
            // );
        }   

        
        Ok(())
    }

    fn can_execute(&self, _ctx: &Context, _interaction: &ComponentInteraction) -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct IgnoreButton {
    pub name: String,
    pub style: ButtonStyle,
    pub label: String,
}

impl IgnoreButton {
    pub fn new() -> Self {
        Self { name: "Ignore".to_string(), style: ButtonStyle::Primary, label: "Ignore".to_string() }
    }
}

#[async_trait]
impl InteractionButton for IgnoreButton {
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

    async fn execute(
        &mut self,
        _ctx: &Context,
        interaction: &ComponentInteraction,
    ) -> Result<(), PoiseError> {
        debug!(
            "Ignore Button {} pressed by {}.",
            self.name, interaction.user.name
        );

        Ok(())
    }

    fn can_execute(&self, _ctx: &Context, _interaction: &ComponentInteraction) -> bool {
        true
    }
}