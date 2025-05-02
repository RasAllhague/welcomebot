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
