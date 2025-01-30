use poise::serenity_prelude::{self as serenity, Color, CreateEmbedAuthor, Timestamp};

pub trait ToEmbed {
    fn to_embed(&self) -> serenity::CreateEmbed;
}

#[derive(Clone, Debug)]
pub struct BanEmbed {
    pub user_id: i64,
    pub user_name: String,
    pub icon_url: String,
    pub reason: Option<String>,
    pub bot_name: String,
    pub unbanned_by: Option<String>,
}

impl BanEmbed {
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
    pub user_id: u64,
    pub user_name: String,
    pub icon_url: String,
    pub timestamp: Timestamp,
}

impl SuspiciousUserEmbed {
    pub const fn new(
        user_id: u64,
        user_name: String,
        icon_url: String,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            user_id,
            user_name,
            icon_url,
            timestamp,
        }
    }
}

impl ToEmbed for SuspiciousUserEmbed {
    fn to_embed(&self) -> serenity::CreateEmbed {
        serenity::CreateEmbed::new()
            .title(format!("Suspicious user: {}", self.user_name))
            .description("User has been flagged as suspicious.")
            .field("Id", self.user_id.to_string(), true)
            .field("Flagged at", self.timestamp.to_string(), true)
            .author(CreateEmbedAuthor::new("Moderation Bot").icon_url(&self.icon_url))
            .color(Color::DARK_GREEN)
            .timestamp(Timestamp::now())
    }
}
