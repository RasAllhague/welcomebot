use ::entity::ban_entry::{self};
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
pub async fn create(db: &DbConn, new_model: ban_entry::Model) -> Result<ban_entry::Model, DbErr> {
    ban_entry::ActiveModel {
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
    ban_entry::Entity::delete_many()
        .filter(
            ban_entry::Column::GuildId
                .eq(guild_id)
                .and(ban_entry::Column::UserId.eq(user_id)),
        )
        .exec(db)
        .await
}
