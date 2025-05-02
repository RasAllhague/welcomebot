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
