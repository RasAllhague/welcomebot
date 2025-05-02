use ::entity::ban_entry::{self, Entity as BanEntry};
use sea_orm::*;

/// Retrieves all ban entries for a specific guild.
///
/// # Arguments
/// * `db` - The database connection.
/// * `guild_id` - The ID of the guild to retrieve ban entries for.
///
/// # Returns
/// Returns a `Vec` containing all ban entry models for the specified guild.
///
/// # Errors
/// Returns a [`DbErr`] if the database operation fails.
#[fastrace::trace]
pub async fn get_all(db: &DbConn, guild_id: i32) -> Result<Vec<ban_entry::Model>, DbErr> {
    BanEntry::find()
        .filter(ban_entry::Column::GuildId.eq(guild_id))
        .all(db)
        .await
}
