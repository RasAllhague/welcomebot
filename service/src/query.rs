pub mod guild_query {
    use ::entity::guild::{self, Entity as Guild};
    use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

    /// Retrieves a guild by its ID.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `id` - The ID of the guild to retrieve.
    ///
    /// # Returns
    /// Returns an [`Option`] containing the guild model if found, or `None` if no guild is found.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<guild::Model>, DbErr> {
        Guild::find_by_id(id).one(db).await
    }

    /// Retrieves a guild by its guild ID.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `guild_id` - The guild ID to search for.
    ///
    /// # Returns
    /// Returns an [`Option`] containing the guild model if found, or `None` if no guild is found.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn get_by_guild_id(
        db: &DbConn,
        guild_id: i64,
    ) -> Result<Option<guild::Model>, DbErr> {
        Guild::find()
            .filter(guild::Column::GuildId.eq(guild_id))
            .one(db)
            .await
    }
}

pub mod image_query {
    use ::entity::image::{self, Entity as Image};
    use sea_orm::{DbConn, DbErr, EntityTrait};

    /// Retrieves an image by its ID.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `image_id` - The ID of the image to retrieve.
    ///
    /// # Returns
    /// Returns an [`Option`] containing the image model if found, or `None` if no image is found.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn get_one(db: &DbConn, image_id: i32) -> Result<Option<image::Model>, DbErr> {
        Image::find_by_id(image_id).one(db).await
    }
}

pub mod welcome_settings_query {
    use ::entity::welcome_settings::{self, Entity as WelcomeSettings};
    use sea_orm::{DbConn, DbErr, EntityTrait};

    /// Retrieves welcome settings by their ID.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `id` - The ID of the welcome settings to retrieve.
    ///
    /// # Returns
    /// Returns an [`Option`] containing the welcome settings model if found, or `None` if no settings are found.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<welcome_settings::Model>, DbErr> {
        WelcomeSettings::find_by_id(id).one(db).await
    }
}

pub mod ban_entry_query {
    use ::entity::ban_entry::{self, Entity as BanEntry};
    use sea_orm::*;

    /// Retrieves all ban entries for a specific guild.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `guild_id` - The ID of the guild to retrieve ban entries for.
    ///
    /// # Returns
    /// Returns a `Vec` containing all ban entry models for the specified guild.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn get_all(db: &DbConn, guild_id: i32) -> Result<Vec<ban_entry::Model>, DbErr> {
        BanEntry::find()
            .filter(ban_entry::Column::GuildId.eq(guild_id))
            .all(db)
            .await
    }
}

pub mod twitch_broadcaster_query {
    use ::entity::twitch_broadcaster::{self, Entity as TwitchBroadcaster};
    use sea_orm::*;

    /// Retrieves the Twitch token from the database.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    ///
    /// # Returns
    /// Returns an [`Option`] containing the Twitch token model if found, or `None` if no token is found.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn get(db: &DbConn) -> Result<Option<twitch_broadcaster::Model>, DbErr> {
        TwitchBroadcaster::find().one(db).await
    }
}
