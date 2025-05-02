use ::entity::image::{self, Entity as Image};
use sea_orm::{DbConn, DbErr, EntityTrait};

/// Retrieves an image by its ID.
///
/// # Arguments
/// * `db` - The database connection.
/// * `image_id` - The ID of the image to retrieve.
///
/// # Returns
/// Returns an [`Option`] containing the image model if found, or `None` if no image is found.
///
/// # Errors
/// Returns a [`DbErr`] if the database operation fails.
#[fastrace::trace]
pub async fn get_one(db: &DbConn, image_id: i32) -> Result<Option<image::Model>, DbErr> {
    Image::find_by_id(image_id).one(db).await
}
