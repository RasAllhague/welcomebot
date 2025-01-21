pub mod guild_query {
    use ::entity::guild::{self, Entity as Guild};

    use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<guild::Model>, DbErr> {
        Guild::find_by_id(id).one(db).await
    }

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

    pub async fn get_one(db: &DbConn, image_id: i32) -> Result<Option<image::Model>, DbErr> {
        Image::find_by_id(image_id).one(db).await
    }
}

pub mod welcome_settings_query {
    use ::entity::welcome_settings::{self, Entity as WelcomeSettings};

    use sea_orm::{DbConn, DbErr, EntityTrait};

    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<welcome_settings::Model>, DbErr> {
        WelcomeSettings::find_by_id(id).one(db).await
    }
}

pub mod ban_entry_query {
    use ::entity::ban_entry::{self, Entity as BanEntry};
    use sea_orm::*;

    pub async fn get_all(db: &DbConn, guild_id: i32) -> Result<Vec<ban_entry::Model>, DbErr> {
        BanEntry::find()
            .filter(ban_entry::Column::GuildId.eq(guild_id))
            .all(db)
            .await
    }

    pub async fn is_not_banned(db: &DbConn, guild_id: i32, user_id: i64) -> Result<bool, DbErr> {
        Ok(BanEntry::find()
            .filter(
                ban_entry::Column::GuildId
                    .eq(guild_id)
                    .and(ban_entry::Column::UserId.eq(user_id)),
            )
            .order_by_desc(ban_entry::Column::CreateDate)
            .all(db)
            .await?
            .is_empty())
    }
}
