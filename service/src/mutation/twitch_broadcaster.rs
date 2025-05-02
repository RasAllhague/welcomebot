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
        match crate::query::twitch_broadcaster::get_by_broadcaster_id(
            db,
            &update_model.broadcaster_id,
        )
        .await?
        {
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
