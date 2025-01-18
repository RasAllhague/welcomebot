pub mod guild_query {
    use ::entity::guild::{self, Entity as Guild};

    use sea_orm::*;

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

    use sea_orm::*;

    pub async fn get_one(db: &DbConn, image_id: i32) -> Result<Option<image::Model>, DbErr> {
        Image::find_by_id(image_id).one(db).await
    }
}

pub mod welcome_settings_query {
    use ::entity::welcome_settings::{self, Entity as WelcomeSettings};

    use sea_orm::*;

    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<welcome_settings::Model>, DbErr> {
        WelcomeSettings::find_by_id(id).one(db).await
    }
}

pub mod auto_ban_role_query {
    use ::entity::{auto_ban_role::{self, Entity as AutoBanRole}, guild};

    use sea_orm::*;

    pub async fn get_by_role_id(
        db: &DbConn,
        guild_id: i64,
        role_id: i64,
    ) -> Result<Option<auto_ban_role::Model>, DbErr> {
        AutoBanRole::find()
            .left_join(::entity::guild::Entity)
            .filter(
                auto_ban_role::Column::RoleId
                    .eq(role_id)
                    .and(guild::Column::GuildId.eq(guild_id)),
            )
            .one(db)
            .await
    }
}
