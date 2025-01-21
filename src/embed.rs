use poise::serenity_prelude::{self as serenity, Color, CreateEmbedAuthor, Timestamp};

pub struct BanEmbed {
    pub user_id: i64,
    pub display_name: String,
    pub user_name: String,
    pub icon_url: String,
    pub reason: String,
    pub bot_name: String,
}

impl BanEmbed {
    pub fn new(user_id: i64, display_name: String, user_name: String, icon_url: String, reason: String, bot_name: String) -> Self {
        Self {
            user_id,
            display_name,
            user_name,
            icon_url,
            reason,
            bot_name,
        }
    }

    pub fn to_embed(&self) -> serenity::CreateEmbed {
        serenity::CreateEmbed::new()
            .title(format!("User banned: {}", self.display_name))
            .description(format!("Banned for: {}", self.reason))
            .field("Id", self.user_id.to_string(), true)
            .field("Username", self.user_name.to_string(), true)
            .author(CreateEmbedAuthor::new(&self.bot_name).icon_url(&self.icon_url))
            .color(Color::RED)
            .timestamp(Timestamp::now())
    }
}
