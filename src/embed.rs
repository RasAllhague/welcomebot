use poise::serenity_prelude::{self as serenity, Color, CreateEmbedAuthor, Timestamp};

pub struct BanEmbed {
    pub user_id: i64,
    pub user_name: String,
    pub icon_url: String,
    pub reason: Option<String>,
    pub bot_name: String,
}

impl BanEmbed {
    pub fn new(
        user_id: i64,
        user_name: String,
        icon_url: String,
        reason: Option<String>,
        bot_name: String,
    ) -> Self {
        Self {
            user_id,
            user_name,
            icon_url,
            reason,
            bot_name,
        }
    }

    pub fn to_embed(&self) -> serenity::CreateEmbed {
        serenity::CreateEmbed::new()
            .title(format!("User banned: {}", self.user_name))
            .description(format!(
                "Banned for: {}",
                self.reason
                    .clone()
                    .unwrap_or(String::from("No reason given."))
            ))
            .field("Id", self.user_id.to_string(), true)
            .author(CreateEmbedAuthor::new(&self.bot_name).icon_url(&self.icon_url))
            .color(Color::RED)
            .timestamp(Timestamp::now())
    }
}
