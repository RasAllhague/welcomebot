use ::entity::twitch_broadcaster::{self, Entity as TwitchBroadcaster};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};

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
        match crate::twitch_broadcaster::get_by_broadcaster_id(db, &update_model.broadcaster_id)
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
pub async fn get_by_broadcaster_id(
    db: &DbConn,
    broadcaster_id: &str,
) -> Result<Option<twitch_broadcaster::Model>, DbErr> {
    TwitchBroadcaster::find()
        .filter(twitch_broadcaster::Column::BroadcasterId.eq(broadcaster_id))
        .one(db)
        .await
}

#[fastrace::trace]
pub async fn get_by_broadcaster_login(
    db: &DbConn,
    broadcaster_login: &str,
) -> Result<Option<twitch_broadcaster::Model>, DbErr> {
    TwitchBroadcaster::find()
        .filter(twitch_broadcaster::Column::BroadcasterLogin.eq(broadcaster_login))
        .one(db)
        .await
}

#[fastrace::trace]
pub async fn get_all(db: &DbConn) -> Result<Vec<twitch_broadcaster::Model>, DbErr> {
    TwitchBroadcaster::find().all(db).await
}
