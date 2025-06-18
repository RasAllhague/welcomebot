use poise::serenity_prelude::{
    self as serenity, Color, CreateEmbedAuthor, Timestamp,
};

/// Trait for converting a struct to a Discord embed.
///
/// This trait provides a method for converting a struct into a `CreateEmbed`
/// instance, which can be used to send rich embeds in Discord messages.
pub trait ToEmbed {
    /// Converts the struct to a `CreateEmbed` instance.
    ///
    /// # Returns
    /// A `CreateEmbed` instance representing the struct as a Discord embed.
    fn to_embed(&self) -> serenity::CreateEmbed;
}

/// Represents an embed for a banned user.
///
/// This embed is used to display information about a banned user, including
/// their ID, name, reason for the ban, and the bot that issued the ban.
#[derive(Clone, Debug)]
pub struct BanEmbed {
    /// The ID of the banned user.
    pub user_id: i64,
    /// The name of the banned user.
    pub user_name: String,
    /// The URL of the user's icon.
    pub icon_url: String,
    /// The reason for the ban.
    pub reason: Option<String>,
    /// The name of the bot that issued the ban.
    pub bot_name: String,
    /// The name of the user who unbanned the banned user, if applicable.
    pub unbanned_by: Option<String>,
}

impl BanEmbed {
    /// Creates a new `BanEmbed` instance.
    ///
    /// # Arguments
    /// * `user_id` - The ID of the banned user.
    /// * `user_name` - The name of the banned user.
    /// * `icon_url` - The URL of the user's icon.
    /// * `reason` - The reason for the ban.
    /// * `bot_name` - The name of the bot that issued the ban.
    /// * `unbanned_by` - The name of the user who unbanned the banned user, if applicable.
    ///
    /// # Returns
    /// A new `BanEmbed` instance.
    pub const fn new(
        user_id: i64,
        user_name: String,
        icon_url: String,
        reason: Option<String>,
        bot_name: String,
        unbanned_by: Option<String>,
    ) -> Self {
        Self {
            user_id,
            user_name,
            icon_url,
            reason,
            bot_name,
            unbanned_by,
        }
    }
}

impl ToEmbed for BanEmbed {
    /// Converts the `BanEmbed` instance to a `CreateEmbed` instance.
    ///
    /// # Returns
    /// A `CreateEmbed` instance representing the banned user.
    fn to_embed(&self) -> serenity::CreateEmbed {
        let mut embed = serenity::CreateEmbed::new()
            .title(format!("User banned: {}", self.user_name))
            .description(format!(
                "Banned for: {}",
                self.reason
                    .clone()
                    .unwrap_or_else(|| String::from("No reason given."))
            ))
            .field("Id", self.user_id.to_string(), true)
            .author(CreateEmbedAuthor::new(&self.bot_name).icon_url(&self.icon_url))
            .color(Color::RED)
            .timestamp(Timestamp::now());

        if let Some(unbanned_by) = &self.unbanned_by {
            embed = embed.field("Unbanned by", unbanned_by, true);
        }

        embed
    }
}

/// Represents an embed for a suspicious user.
///
/// This embed is used to display information about a user flagged as suspicious,
/// including their ID, name, and the time they were flagged.
#[derive(Debug, Clone)]
pub struct SuspiciousUserEmbed {
    /// The name of the bot that flagged the user.
    bot_name: String,
    /// The ID of the suspicious user.
    user_id: u64,
    /// The name of the suspicious user.
    user_name: String,
    /// The URL of the user's icon.
    icon_url: String,
    /// The timestamp when the user was flagged.
    timestamp: Timestamp,
}

impl SuspiciousUserEmbed {
    /// Creates a new `SuspiciousUserEmbed` instance.
    ///
    /// # Arguments
    /// * `bot_name` - The name of the bot that flagged the user.
    /// * `user_id` - The ID of the suspicious user.
    /// * `user_name` - The name of the suspicious user.
    /// * `icon_url` - The URL of the user's icon.
    /// * `timestamp` - The timestamp when the user was flagged.
    ///
    /// # Returns
    /// A new `SuspiciousUserEmbed` instance.
    pub const fn new(
        bot_name: String,
        user_id: u64,
        user_name: String,
        icon_url: String,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            bot_name,
            user_id,
            user_name,
            icon_url,
            timestamp,
        }
    }

    /// Returns the name of the bot that flagged the user.
    pub fn bot_name(&self) -> &str {
        &self.bot_name
    }

    /// Returns the ID of the suspicious user.
    pub fn user_id(&self) -> u64 {
        self.user_id
    }

    /// Returns the name of the suspicious user.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Returns the URL of the user's icon.
    pub fn icon_url(&self) -> &str {
        &self.icon_url
    }

    /// Returns the timestamp when the user was flagged.
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

impl ToEmbed for SuspiciousUserEmbed {
    /// Converts the `SuspiciousUserEmbed` instance to a `CreateEmbed` instance.
    ///
    /// # Returns
    /// A `CreateEmbed` instance representing the suspicious user.
    fn to_embed(&self) -> serenity::CreateEmbed {
        serenity::CreateEmbed::new()
            .title(format!("Suspicious user: {}", self.user_name))
            .description("User has been flagged as suspicious.")
            .field("Id", self.user_id.to_string(), true)
            .field("Flagged at", self.timestamp.to_string(), true)
            .author(CreateEmbedAuthor::new(&self.bot_name).icon_url(&self.icon_url))
            .color(Color::DARK_GREEN)
            .timestamp(Timestamp::now())
    }
}

#[derive(Debug, Clone)]
pub struct KickLogEmbed {
    kicked_user_name: String,
    kicked_user_id: u64,
    create_user_name: String,
    create_user_id: u64,
    create_user_icon: String,
    reason: Option<String>,
    timestamp: Timestamp,
}

impl KickLogEmbed {
    pub fn new(
        kicked_user_name: String,
        kicked_user_id: u64,
        create_user_name: String,
        create_user_id: u64,
        create_user_icon: String,
        reason: Option<String>,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            kicked_user_name,
            kicked_user_id,
            create_user_name,
            create_user_id,
            create_user_icon,
            reason,
            timestamp,
        }
    }

    pub fn kicked_user_name(&self) -> &str {
        &self.kicked_user_name
    }

    pub fn kicked_user_id(&self) -> u64 {
        self.kicked_user_id
    }

    pub fn create_user_name(&self) -> &str {
        &self.create_user_name
    }

    pub fn create_user_id(&self) -> u64 {
        self.create_user_id
    }

    pub fn create_user_icon(&self) -> &str {
        &self.create_user_icon
    }

    pub fn reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

impl ToEmbed for KickLogEmbed {
    fn to_embed(&self) -> serenity::CreateEmbed {
        serenity::CreateEmbed::new()
            .title(format!(
                "{}({}) kicked {}({})",
                self.create_user_name(),
                self.create_user_id(),
                self.kicked_user_name(),
                self.kicked_user_id()
            ))
            .description(format!("- With reason **{}**", self.reason().unwrap_or(&"No reason".to_owned())))
            .timestamp(self.timestamp())
            .author(CreateEmbedAuthor::new(self.create_user_name()).icon_url(self.create_user_icon()))
    }
}
