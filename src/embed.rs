use poise::serenity_prelude::{self as serenity, Color, CreateEmbedAuthor, Timestamp};

pub struct BanEmbed {
    pub banned_id: i64,
    pub banned_name: String,
    pub reason: String,
    pub bot_name: String,
}

impl BanEmbed {
    pub fn new(banned_id: i64, banned_name: String, reason: String, bot_name: String) -> Self {
        Self {
            banned_id,
            banned_name,
            reason,
            bot_name,
        }
    }

    pub fn to_embed(&self) -> serenity::CreateEmbed {
        serenity::CreateEmbed::new()
            .title(format!("User banned: {}", self.banned_name))
            .description(format!("Banned for: {}", self.reason))
            .field("Id", self.banned_id.to_string(), false)
            .author(CreateEmbedAuthor::new(&self.bot_name))
            .color(Color::RED)
            .timestamp(Timestamp::now())
    }
}
