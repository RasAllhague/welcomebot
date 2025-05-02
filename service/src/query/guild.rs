use ::entity::guild::{self, Entity as Guild};
use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

/// Retrieves a guild by its ID.
///
/// # Arguments
/// * `db` - The database connection.
/// * `id` - The ID of the guild to retrieve.
///
/// # Returns
/// Returns an [`Option`] containing the guild model if found, or `None` if no guild is found.
///
/// # Errors
/// Returns a [`DbErr`] if the database operation fails.
#[fastrace::trace]
pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<guild::Model>, DbErr> {
    Guild::find_by_id(id).one(db).await
}

/// Retrieves a guild by its guild ID.
///
/// # Arguments
/// * `db` - The database connection.
/// * `guild_id` - The guild ID to search for.
///
/// # Returns
/// Returns an [`Option`] containing the guild model if found, or `None` if no guild is found.
///
/// # Errors
/// Returns a [`DbErr`] if the database operation fails.
#[fastrace::trace]
pub async fn get_by_guild_id(db: &DbConn, guild_id: i64) -> Result<Option<guild::Model>, DbErr> {
    Guild::find()
        .filter(guild::Column::GuildId.eq(guild_id))
        .one(db)
        .await
}
