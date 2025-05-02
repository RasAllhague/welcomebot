use ::entity::image::{self, Entity as Image};
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, Set};

/// Creates a new image entry in the database.
///
/// # Arguments
/// * `db` - The database connection.
/// * `image` - The image model to insert.
///
/// # Errors
/// Returns a [`DbErr`] if the database operation fails.
#[fastrace::trace]
pub async fn create(db: &DbConn, image: image::Model) -> Result<image::Model, DbErr> {
    image::ActiveModel {
        original_name: Set(image.original_name),
        server_name: Set(image.server_name),
        path: Set(image.path),
        width: Set(image.width),
        height: Set(image.height),
        size: Set(image.size),
        create_user_id: Set(image.create_user_id),
        create_date: Set(image.create_date),
        ..Default::default()
    }
    .insert(db)
    .await
}

/// Updates an existing image entry in the database.
///
/// # Arguments
/// * `db` - The database connection.
/// * `update_image` - The updated image model.
///
/// # Errors
/// Returns a [`DbErr`] if the database operation fails.
#[fastrace::trace]
pub async fn update(
    db: &DbConn,
    update_image: image::Model,
) -> Result<Option<image::Model>, DbErr> {
    let image: image::ActiveModel = match Image::find_by_id(update_image.id).one(db).await? {
        Some(m) => m.into(),
        None => return Ok(None),
    };

    let updated = image::ActiveModel {
        id: image.id,
        original_name: Set(update_image.original_name),
        server_name: Set(update_image.server_name),
        path: Set(update_image.path),
        width: Set(update_image.width),
        height: Set(update_image.height),
        size: Set(update_image.size),
        create_date: image.create_date,
        create_user_id: image.create_user_id,
    }
    .update(db)
    .await?;

    Ok(Some(updated))
}
