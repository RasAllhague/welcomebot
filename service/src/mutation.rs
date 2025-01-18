pub mod image_mutation {
    use ::entity::image::{self, Entity as Image};

    use sea_orm::*;

    /// Creates a new gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
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

    /// Updates the details of a gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
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

    use sea_orm::*;

    /// Creates a new gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn create(db: &DbConn, guild: guild::Model) -> Result<guild::Model, DbErr> {
        guild::ActiveModel {
            name: Set(guild.name),
            guild_id: Set(guild.guild_id),
            welcome_settings_id: Set(guild.welcome_settings_id),
            create_user_id: Set(guild.create_user_id),
            create_date: Set(guild.create_date),
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
    pub async fn update(
        db: &DbConn,
        update_guild: guild::Model,
    ) -> Result<Option<guild::Model>, DbErr> {
        let guild: guild::ActiveModel = match Guild::find_by_id(update_guild.id).one(db).await? {
            Some(m) => m.into(),
            None => return Ok(None),
        };

        let updated = guild::ActiveModel {
            id: guild.id,
            name: Set(update_guild.name),
            guild_id: Set(update_guild.guild_id),
            welcome_settings_id: Set(update_guild.welcome_settings_id),
            create_date: guild.create_date,
            create_user_id: guild.create_user_id,
            modify_date: Set(update_guild.modify_date),
            modify_user_id: Set(update_guild.modify_user_id),
        }
        .update(db)
        .await?;

        Ok(Some(updated))
    }
}

pub mod welcome_settings_mutation {
    use ::entity::welcome_settings::{self, Entity as WelcomeSettings};

    use sea_orm::*;

    /// Creates a new gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
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

pub mod auto_ban_role_mutation {
    use ::entity::auto_ban_role::{self, Entity as AutoBanRole};

    use sea_orm::*;

    pub async fn create(
        db: &DbConn,
        auto_ban_role: auto_ban_role::Model,
    ) -> Result<auto_ban_role::Model, DbErr> {
        auto_ban_role::ActiveModel {
            role_id: Set(auto_ban_role.role_id),
            guild_id: Set(auto_ban_role.guild_id),
            create_user_id: Set(auto_ban_role.create_user_id),
            create_date: Set(auto_ban_role.create_date),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        AutoBanRole::delete_by_id(id).exec(db).await
    }

    pub async fn delete_by_role(
        db: &DbConn,
        guild_id: i32,
        role_id: i64,
    ) -> Result<DeleteResult, DbErr> {
        AutoBanRole::delete_many()
            .filter(
                auto_ban_role::Column::RoleId
                    .eq(role_id)
                    .and(auto_ban_role::Column::GuildId.eq(guild_id)),
            )
            .exec(db)
            .await
    }
}
