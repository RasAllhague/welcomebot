pub mod image_mutation {
    use ::entity::image::{self, Entity as Image};
    use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, Set};

    /// Creates a new image entry in the database.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `image` - The image model to insert.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn create(db: &DbConn, image: image::Model) -> Result<image::Model, DbErr> {
        image::ActiveModel {
            original_name: Set(image.original_name),
            server_name: Set(image.server_name),
            path: Set(image.path),
            width: Set(image.width),
            height: Set(image.height),
            size: Set(image.size),
            create_user_id: Set(image.create_user_id),
            create_date: Set(image.create_date),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// Updates an existing image entry in the database.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `update_image` - The updated image model.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn update(
        db: &DbConn,
        update_image: image::Model,
    ) -> Result<Option<image::Model>, DbErr> {
        let image: image::ActiveModel = match Image::find_by_id(update_image.id).one(db).await? {
            Some(m) => m.into(),
            None => return Ok(None),
        };

        let updated = image::ActiveModel {
            id: image.id,
            original_name: Set(update_image.original_name),
            server_name: Set(update_image.server_name),
            path: Set(update_image.path),
            width: Set(update_image.width),
            height: Set(update_image.height),
            size: Set(update_image.size),
            create_date: image.create_date,
            create_user_id: image.create_user_id,
        }
        .update(db)
        .await?;

        Ok(Some(updated))
    }
}

pub mod guild_mutation {
    use ::entity::guild::{self, Entity as Guild};
    use chrono::Utc;

    use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, Set};

    /// Creates a new gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    #[fastrace::trace]
    pub async fn create(db: &DbConn, guild: guild::Model) -> Result<guild::Model, DbErr> {
        guild::ActiveModel {
            name: Set(guild.name),
            guild_id: Set(guild.guild_id),
            moderation_channel_id: Set(guild.moderation_channel_id),
            welcome_settings_id: Set(guild.welcome_settings_id),
            auto_ban_role_id: Set(guild.auto_ban_role_id),
            ban_reason_template: Set(guild.ban_reason_template),
            create_user_id: Set(guild.create_user_id),
            create_date: Set(guild.create_date),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    #[fastrace::trace]
    pub async fn get_or_create<T: AsRef<str> + std::marker::Send>(
        db: &DbConn,
        guild_id: i64,
        guild_name: T,
        create_user_id: i64,
    ) -> Result<guild::Model, DbErr> {
        if let Some(g) = crate::guild_query::get_by_guild_id(db, guild_id).await? {
            Ok(g)
        } else {
            let guild = guild::Model {
                id: 0,
                name: guild_name.as_ref().to_string(),
                guild_id,
                welcome_settings_id: None,
                moderation_channel_id: None,
                auto_ban_role_id: None,
                ban_reason_template: None,
                punish_mode: "kick".to_string(),
                create_user_id,
                create_date: Utc::now().naive_utc().to_string(),
                modify_date: None,
                modify_user_id: None,
            };

            create(db, guild).await
        }
    }

    /// Updates the details of a gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    #[fastrace::trace]
    pub async fn update(
        db: &DbConn,
        update_guild: &guild::Model,
    ) -> Result<Option<guild::Model>, DbErr> {
        let guild: guild::ActiveModel = match Guild::find_by_id(update_guild.id).one(db).await? {
            Some(m) => m.into(),
            None => return Ok(None),
        };

        let updated = guild::ActiveModel {
            id: guild.id,
            name: Set(update_guild.name.clone()),
            guild_id: Set(update_guild.guild_id),
            moderation_channel_id: Set(update_guild.moderation_channel_id),
            welcome_settings_id: Set(update_guild.welcome_settings_id),
            auto_ban_role_id: Set(update_guild.auto_ban_role_id),
            punish_mode: Set(update_guild.punish_mode.clone()),
            ban_reason_template: Set(update_guild.ban_reason_template.clone()),
            create_date: guild.create_date,
            create_user_id: guild.create_user_id,
            modify_date: Set(update_guild.modify_date.clone()),
            modify_user_id: Set(update_guild.modify_user_id),
        }
        .update(db)
        .await?;

        Ok(Some(updated))
    }
}

pub mod welcome_settings_mutation {
    use ::entity::welcome_settings::{self, Entity as WelcomeSettings};

    use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, Set};

    /// Creates a new gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    #[fastrace::trace]
    pub async fn create(
        db: &DbConn,
        welcome_settings: welcome_settings::Model,
    ) -> Result<welcome_settings::Model, DbErr> {
        welcome_settings::ActiveModel {
            welcome_channel: Set(welcome_settings.welcome_channel),
            chat_message: Set(welcome_settings.chat_message),
            image_headline: Set(welcome_settings.image_headline),
            image_subtext: Set(welcome_settings.image_subtext),
            front_banner: Set(welcome_settings.front_banner),
            back_banner: Set(welcome_settings.back_banner),
            create_user_id: Set(welcome_settings.create_user_id),
            create_date: Set(welcome_settings.create_date),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// Updates the details of a gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    #[fastrace::trace]
    pub async fn update(
        db: &DbConn,
        update_welcome_settings: welcome_settings::Model,
    ) -> Result<Option<welcome_settings::Model>, DbErr> {
        let welcome_settings: welcome_settings::ActiveModel =
            match WelcomeSettings::find_by_id(update_welcome_settings.id)
                .one(db)
                .await?
            {
                Some(m) => m.into(),
                None => return Ok(None),
            };

        let updated = welcome_settings::ActiveModel {
            id: welcome_settings.id,
            welcome_channel: Set(update_welcome_settings.welcome_channel),
            chat_message: Set(update_welcome_settings.chat_message),
            image_headline: Set(update_welcome_settings.image_headline),
            image_subtext: Set(update_welcome_settings.image_subtext),
            front_banner: Set(update_welcome_settings.front_banner),
            back_banner: Set(update_welcome_settings.back_banner),
            enabled: Set(update_welcome_settings.enabled),
            create_date: welcome_settings.create_date,
            create_user_id: welcome_settings.create_user_id,
            modify_date: Set(update_welcome_settings.modify_date),
            modify_user_id: Set(update_welcome_settings.modify_user_id),
        }
        .update(db)
        .await?;

        Ok(Some(updated))
    }
}

pub mod ban_entry_mutation {
    use ::entity::ban_entry::{self};
    use sea_orm::{
        ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait, QueryFilter, Set,
    };

    /// Creates a new ban entry in the database.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `new_model` - The ban entry model to insert.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn create(
        db: &DbConn,
        new_model: ban_entry::Model,
    ) -> Result<ban_entry::Model, DbErr> {
        ban_entry::ActiveModel {
            guild_id: Set(new_model.guild_id),
            user_id: Set(new_model.user_id),
            user_name: Set(new_model.user_name),
            reason: Set(new_model.reason),
            create_user_id: Set(new_model.create_user_id),
            create_date: Set(new_model.create_date),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// Deletes a ban entry by user ID in the database.
    ///
    /// # Arguments
    /// * `db` - The database connection.
    /// * `guild_id` - The guild ID associated with the ban entry.
    /// * `user_id` - The user ID of the ban entry to delete.
    ///
    /// # Errors
    /// Returns a [`DbErr`] if the database operation fails.
    #[fastrace::trace]
    pub async fn delete_by_user_id(
        db: &DbConn,
        guild_id: i32,
        user_id: i64,
    ) -> Result<DeleteResult, DbErr> {
        ban_entry::Entity::delete_many()
            .filter(
                ban_entry::Column::GuildId
                    .eq(guild_id)
                    .and(ban_entry::Column::UserId.eq(user_id)),
            )
            .exec(db)
            .await
    }
}

pub mod twitch_broadcaster_mutation {
    use crate::twitch_broadcaster_query;
    use ::entity::twitch_broadcaster::{self};
    use chrono::Utc;
    use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

    pub async fn create(
        db: &DbConn,
        new_model: twitch_broadcaster::Model,
    ) -> Result<twitch_broadcaster::Model, DbErr> {
        twitch_broadcaster::ActiveModel {
            broadcaster_login: Set(new_model.broadcaster_login),
            broadcaster_id: Set(new_model.broadcaster_id),
            broadcaster_name: Set(new_model.broadcaster_name),
            access_token: Set(new_model.access_token),
            refresh_token: Set(new_model.refresh_token),
            last_refreshed: Set(new_model.last_refreshed),
            create_date: Set(Utc::now()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn update(
        db: &DbConn,
        update_model: twitch_broadcaster::Model,
    ) -> Result<Option<twitch_broadcaster::Model>, DbErr> {
        let model: twitch_broadcaster::ActiveModel =
            match twitch_broadcaster_query::get_by_broadcaster_id(db, &update_model.broadcaster_id).await? {
                Some(m) => m.into(),
                None => return Ok(None),
            };

        let updated = twitch_broadcaster::ActiveModel {
            id: model.id,
            broadcaster_login: Set(update_model.broadcaster_login),
            broadcaster_id: Set(update_model.broadcaster_id),
            broadcaster_name: Set(update_model.broadcaster_name),
            access_token: Set(update_model.access_token),
            refresh_token: Set(update_model.refresh_token),
            last_refreshed: Set(Some(Utc::now())),
            create_date: model.create_date,
            modify_date: Set(Some(Utc::now())),
        }
        .update(db)
        .await?;

        Ok(Some(updated))
    }
}
