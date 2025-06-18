use ::entity::kick_entry::{self, Entity as KickEntry};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait, QueryFilter, Set,
};

/// Creates a new ban entry in the database.
///
/// # Arguments
/// * `db` - The database connection.
/// * `new_model` - The ban entry model to insert.
///
/// # Errors
/// Returns a [`DbErr`] if the database operation fails.
#[fastrace::trace]
pub async fn create(db: &DbConn, new_model: kick_entry::Model) -> Result<kick_entry::Model, DbErr> {
    kick_entry::ActiveModel {
        guild_id: Set(new_model.guild_id),
        user_id: Set(new_model.user_id),
        user_name: Set(new_model.user_name),
        reason: Set(new_model.reason),
        create_user_id: Set(new_model.create_user_id),
        create_date: Set(new_model.create_date),
        ..Default::default()
    }
    .insert(db)
    .await
}

/// Deletes a ban entry by user ID in the database.
///
/// # Arguments
/// * `db` - The database connection.
/// * `guild_id` - The guild ID associated with the ban entry.
/// * `user_id` - The user ID of the ban entry to delete.
///
/// # Errors
/// Returns a [`DbErr`] if the database operation fails.
#[fastrace::trace]
pub async fn delete_by_user_id(
    db: &DbConn,
    guild_id: i32,
    user_id: i64,
) -> Result<DeleteResult, DbErr> {
    kick_entry::Entity::delete_many()
        .filter(
            kick_entry::Column::GuildId
                .eq(guild_id)
                .and(kick_entry::Column::UserId.eq(user_id)),
        )
        .exec(db)
        .await
}

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
pub async fn get_all(db: &DbConn, guild_id: i32) -> Result<Vec<kick_entry::Model>, DbErr> {
    KickEntry::find()
        .filter(kick_entry::Column::GuildId.eq(guild_id))
        .all(db)
        .await
}
