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
