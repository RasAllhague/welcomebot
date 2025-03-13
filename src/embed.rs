use poise::serenity_prelude::{self as serenity, Color, CreateEmbedAuthor, Timestamp};

/// Trait for converting a struct to a Discord embed.
pub trait ToEmbed {
    /// Converts the struct to a `CreateEmbed` instance.
    fn to_embed(&self) -> serenity::CreateEmbed;
}

/// Represents an embed for a banned user.
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
    ///
    /// * `user_id` - The ID of the banned user.
    /// * `user_name` - The name of the banned user.
    /// * `icon_url` - The URL of the user's icon.
    /// * `reason` - The reason for the ban.
    /// * `bot_name` - The name of the bot that issued the ban.
    /// * `unbanned_by` - The name of the user who unbanned the banned user, if applicable.
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

#[derive(Debug, Clone)]
pub struct SuspiciousUserEmbed {
    bot_name: String,
    user_id: u64,
    user_name: String,
    icon_url: String,
    timestamp: Timestamp,
}

impl SuspiciousUserEmbed {
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
    
    pub fn bot_name(&self) -> &str {
        &self.bot_name
    }
    
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
    
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    
    pub fn icon_url(&self) -> &str {
        &self.icon_url
    }
    
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
}

impl ToEmbed for SuspiciousUserEmbed {
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
