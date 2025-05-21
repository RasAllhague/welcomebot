use async_trait::async_trait;
use log::{debug, info};
use poise::serenity_prelude::{
    ButtonStyle, ComponentInteraction, Context, CreateButton, CreateInteractionResponse,
};
use uuid::Uuid;

use crate::{
    PoiseError,
    embed::{BanEmbed, SuspiciousUserEmbed},
    interaction::InteractionButton,
};

/// Represents a button for kicking a user.
#[derive(Clone, Debug)]
pub struct KickButton {
    /// The name of the button.
    pub name: String,
    /// The style of the button.
    pub style: ButtonStyle,
    /// The label of the button.
    pub label: String,
}

impl KickButton {
    /// Creates a new `KickButton` instance.
    ///
    /// # Arguments
    /// * `interaction_id` - The unique ID of the interaction.
    pub fn new(interaction_id: Uuid) -> Self {
        Self {
            name: format!("{interaction_id}_kick"),
            style: ButtonStyle::Secondary,
            label: "Kick".to_string(),
        }
    }
}

#[async_trait]
impl InteractionButton<SuspiciousUserEmbed> for KickButton {
    /// Returns the name of the button.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the style of the button.
    fn style(&self) -> ButtonStyle {
        self.style
    }

    /// Returns the label of the button.
    fn label(&self) -> String {
        self.label.clone()
    }

    /// Converts the button to a `CreateButton` instance.
    ///
    /// # Arguments
    /// * `is_disabled` - Whether the button should be disabled.
    fn to_create_button(&self, is_disabled: bool) -> CreateButton {
        CreateButton::new(&self.name)
            .style(self.style)
            .label(&self.label)
            .disabled(is_disabled)
    }

    /// Executes the button's action to kick a user.
    ///
    /// # Arguments
    /// * `ctx` - The context in which the button is executed.
    /// * `interaction` - The component interaction that triggered the button.
    /// * `embed` - The embed associated with the button.
    ///
    /// # Errors
    /// Returns a [`PoiseError`] if the kick operation fails.
    #[fastrace::trace]
    async fn execute(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
        embed: &SuspiciousUserEmbed,
    ) -> Result<SuspiciousUserEmbed, PoiseError> {
        if let Some(guild_id) = interaction.guild_id {
            guild_id
                .kick_with_reason(
                    ctx,
                    embed.user_id() as u64,
                    "Kicked by bot for suspicion of spam account.",
                )
                .await?;

            info!(
                "Kicked {}/{} from guild {} by {}/{}",
                embed.user_name(),
                embed.user_id(),
                guild_id,
                interaction.user.name,
                interaction.user.id
            );
        }

        Ok(embed.clone())
    }
}

/// Represents a button for banning a user.
#[derive(Clone, Debug)]
pub struct BanButton {
    /// The name of the button.
    pub name: String,
    /// The style of the button.
    pub style: ButtonStyle,
    /// The label of the button.
    pub label: String,
}

impl BanButton {
    /// Creates a new `BanButton` instance.
    ///
    /// # Arguments
    /// * `interaction_id` - The unique ID of the interaction.
    pub fn new(interaction_id: Uuid) -> Self {
        Self {
            name: format!("{interaction_id}_ban"),
            style: ButtonStyle::Danger,
            label: "Ban".to_string(),
        }
    }
}

#[async_trait]
impl InteractionButton<SuspiciousUserEmbed> for BanButton {
    /// Returns the name of the button.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the style of the button.
    fn style(&self) -> ButtonStyle {
        self.style
    }

    /// Returns the label of the button.
    fn label(&self) -> String {
        self.label.clone()
    }

    /// Converts the button to a `CreateButton` instance.
    ///
    /// # Arguments
    /// * `is_disabled` - Whether the button should be disabled.
    fn to_create_button(&self, is_disabled: bool) -> CreateButton {
        CreateButton::new(&self.name)
            .style(self.style)
            .label(&self.label)
            .disabled(is_disabled)
    }

    /// Executes the button's action to ban a user.
    ///
    /// # Arguments
    /// * `ctx` - The context in which the button is executed.
    /// * `interaction` - The component interaction that triggered the button.
    /// * `embed` - The embed associated with the button.
    ///
    /// # Errors
    /// Returns a [`PoiseError`] if the ban operation fails.
    #[fastrace::trace]
    async fn execute(
        &mut self,
        ctx: &Context,
        interaction: &ComponentInteraction,
        embed: &SuspiciousUserEmbed,
    ) -> Result<SuspiciousUserEmbed, PoiseError> {
        if let Some(guild_id) = interaction.guild_id {
            guild_id
                .ban_with_reason(
                    ctx,
                    embed.user_id() as u64,
                    7,
                    "Banned by bot for suspicion of spam account.",
                )
                .await?;

            info!(
                "Banned {}/{} from guild {} by {}/{}",
                embed.user_name(),
                embed.user_id(),
                guild_id,
                interaction.user.name,
                interaction.user.id
            );
        }

        Ok(embed.clone())
    }
}

/// Represents a button for unbanning a user.
#[derive(Clone, Debug)]
pub struct UnbanButton {
    /// The name of the button.
    pub name: String,
    /// The style of the button.
    pub style: ButtonStyle,
    /// The label of the button.
    pub label: String,
}

impl UnbanButton {
    /// Creates a new `UnbanButton` instance.
    ///
    /// # Arguments
    /// * `interaction_id` - The unique ID of the interaction.
    pub fn new(interaction_id: Uuid) -> Self {
        Self {
            name: format!("{interaction_id}_unban"),
            style: ButtonStyle::Primary,
            label: "Unban".to_string(),
        }
    }
}

#[async_trait]
impl InteractionButton<BanEmbed> for UnbanButton {
    /// Returns the name of the button.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the style of the button.
    fn style(&self) -> ButtonStyle {
        self.style
    }

    /// Returns the label of the button.
    fn label(&self) -> String {
        self.label.clone()
    }

    /// Converts the button to a `CreateButton` instance.
    ///
    /// # Arguments
    /// * `is_disabled` - Whether the button should be disabled.
    fn to_create_button(&self, is_disabled: bool) -> CreateButton {
        CreateButton::new(&self.name)
            .style(self.style)
            .label(&self.label)
            .disabled(is_disabled)
    }

    /// Executes the button's action to unban a user.
    ///
    /// # Arguments
    /// * `ctx` - The context in which the button is executed.
    /// * `interaction` - The component interaction that triggered the button.
    /// * `embed` - The embed associated with the button.
    ///
    /// # Errors
    /// Returns a [`PoiseError`] if the unban operation fails.
    #[fastrace::trace]
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
}

/// Represents a button for ignoring a user.
#[derive(Clone, Debug)]
pub struct IgnoreButton {
    /// The name of the button.
    pub name: String,
    /// The style of the button.
    pub style: ButtonStyle,
    /// The label of the button.
    pub label: String,
}

impl IgnoreButton {
    /// Creates a new `IgnoreButton` instance.
    ///
    /// # Arguments
    /// * `interaction_id` - The unique ID of the interaction.
    pub fn new(interaction_id: Uuid) -> Self {
        Self {
            name: format!("{interaction_id}_ignore"),
            style: ButtonStyle::Primary,
            label: "Ignore".to_string(),
        }
    }
}

#[async_trait]
impl InteractionButton<SuspiciousUserEmbed> for IgnoreButton {
    /// Returns the name of the button.
    fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the style of the button.
    fn style(&self) -> ButtonStyle {
        self.style
    }

    /// Returns the label of the button.
    fn label(&self) -> String {
        self.label.clone()
    }

    /// Converts the button to a `CreateButton` instance.
    ///
    /// # Arguments
    /// * `is_disabled` - Whether the button should be disabled.
    fn to_create_button(&self, is_disabled: bool) -> CreateButton {
        CreateButton::new(&self.name)
            .style(self.style)
            .label(&self.label)
            .disabled(is_disabled)
    }

    /// Executes the button's action to ignore a user.
    ///
    /// # Arguments
    /// * `_ctx` - The context in which the button is executed.
    /// * `interaction` - The component interaction that triggered the button.
    /// * `embed` - The embed associated with the button.
    ///
    /// # Returns
    /// Returns the unchanged embed.
    #[fastrace::trace]
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
}
