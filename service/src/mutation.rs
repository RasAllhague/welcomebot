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
            welcome_message: Set(guild.welcome_message),
            welcome_channel: Set(guild.welcome_channel),
            back_banner: Set(guild.back_banner),
            front_banner: Set(guild.front_banner),
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
            welcome_message: Set(update_guild.welcome_message),
            welcome_channel: Set(update_guild.welcome_channel),
            back_banner: Set(update_guild.back_banner),
            front_banner: Set(update_guild.front_banner),
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
