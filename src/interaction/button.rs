use async_trait::async_trait;
use log::{debug, info};
use poise::serenity_prelude::{
    ButtonStyle, ComponentInteraction, Context, CreateButton, CreateInteractionResponse,
};

use crate::{
    embed::{BanEmbed, SuspiciousUserEmbed},
    interaction::InteractionButton,
    PoiseError,
};

#[derive(Clone, Debug)]
pub struct KickButton {
    pub name: String,
    pub style: ButtonStyle,
    pub label: String,
}

impl KickButton {
    pub fn new() -> Self {
        Self {
            name: "Kick".to_string(),
            style: ButtonStyle::Secondary,
            label: "Kick".to_string(),
        }
    }
}

#[async_trait]
impl InteractionButton<SuspiciousUserEmbed> for KickButton {
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
        embed: &SuspiciousUserEmbed,
    ) -> Result<SuspiciousUserEmbed, PoiseError> {
        if let Some(guild_id) = interaction.guild_id {
            guild_id.kick_with_reason(ctx, embed.user_id as u64, "Kicked by bot for suspicion of spam account.").await?;

            info!(
                "Banned {}/{} from guild {} by {}/{}",
                embed.user_name,
                embed.user_id,
                guild_id,
                interaction.user.name,
                interaction.user.id
            );
        }

        Ok(embed.clone())
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
        Self {
            name: "Ban".to_string(),
            style: ButtonStyle::Danger,
            label: "Ban".to_string(),
        }
    }
}

#[async_trait]
impl InteractionButton<SuspiciousUserEmbed> for BanButton {
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
        embed: &SuspiciousUserEmbed,
    ) -> Result<SuspiciousUserEmbed, PoiseError> {
        if let Some(guild_id) = interaction.guild_id {
            guild_id.ban_with_reason(ctx, embed.user_id as u64, 7, "Banned by bot for suspicion of spam account.").await?;

            info!(
                "Banned {}/{} from guild {} by {}/{}",
                embed.user_name,
                embed.user_id,
                guild_id,
                interaction.user.name,
                interaction.user.id
            );
        }

        Ok(embed.clone())
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
        Self {
            name: "Unban".to_string(),
            style: ButtonStyle::Primary,
            label: "Unban".to_string(),
        }
    }
}

#[async_trait]
impl InteractionButton<BanEmbed> for UnbanButton {
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
        embed: &BanEmbed,
    ) -> Result<BanEmbed, PoiseError> {
        let mut embed = embed.clone();

        if let Some(guild_id) = interaction.guild_id {
            guild_id.unban(ctx, embed.user_id as u64).await?;
            embed.unbanned_by = Some(interaction.user.name.clone());

            interaction
                .create_response(ctx, CreateInteractionResponse::Acknowledge)
                .await?;

            info!(
                "Unbanned {}/{} from guild {} by {}/{}",
                embed.user_name,
                embed.user_id,
                guild_id,
                interaction.user.name,
                interaction.user.id
            );
        }

        Ok(embed.clone())
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
        Self {
            name: "Ignore".to_string(),
            style: ButtonStyle::Primary,
            label: "Ignore".to_string(),
        }
    }
}

#[async_trait]
impl InteractionButton<SuspiciousUserEmbed> for IgnoreButton {
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
        embed: &SuspiciousUserEmbed,
    ) -> Result<SuspiciousUserEmbed, PoiseError> {
        debug!(
            "Ignore Button {} pressed by {}.",
            self.name, interaction.user.name
        );

        Ok(embed.clone())
    }

    fn can_execute(&self, _ctx: &Context, _interaction: &ComponentInteraction) -> bool {
        true
    }
}
