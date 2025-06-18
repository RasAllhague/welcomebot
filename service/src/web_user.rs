use ::entity::web_user::{self, Entity as WebUser};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};

pub async fn create(db: &DbConn, new_model: web_user::Model) -> Result<web_user::Model, DbErr> {
    web_user::ActiveModel {
        id: Set(new_model.id),
        username: Set(new_model.username),
        user_id: Set(new_model.user_id),
        access_token: Set(new_model.access_token),
        last_refresh: Set(new_model.last_refresh),
        password: Set(new_model.password),
        email: Set(new_model.email),
        last_login_ip: Set(new_model.last_login_ip),
        twitch_broadcaster_id: Set(new_model.twitch_broadcaster_id),
        create_date: Set(new_model.create_date),
        ..Default::default()
    }
    .insert(db)
    .await
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
pub async fn get_by_user_id(db: &DbConn, user_id: i64) -> Result<Option<web_user::Model>, DbErr> {
    WebUser::find()
        .filter(web_user::Column::UserId.eq(user_id))
        .one(db)
        .await
}

pub async fn update(
    db: &DbConn,
    update_model: web_user::Model,
) -> Result<Option<web_user::Model>, DbErr> {
    let model: web_user::ActiveModel =
        match crate::web_user::get_by_user_id(db, update_model.user_id).await? {
            Some(m) => m.into(),
            None => return Ok(None),
        };

    let updated = web_user::ActiveModel {
        id: model.id,
        username: Set(update_model.username),
        user_id: model.user_id,
        access_token: Set(update_model.access_token),
        last_refresh: Set(update_model.last_refresh),
        password: Set(update_model.password),
        email: Set(update_model.email),
        last_login_ip: Set(update_model.last_login_ip),
        twitch_broadcaster_id: Set(update_model.twitch_broadcaster_id),
        create_date: model.create_date,
        modify_date: Set(update_model.modify_date),
    }
    .update(db)
    .await?;

    Ok(Some(updated))
}

pub async fn create_or_update(
    db: &DbConn,
    model: web_user::Model,
) -> Result<web_user::Model, DbErr> {
    if let Some(old_model) = get_by_user_id(db, model.user_id).await? {
        let old_model: web_user::ActiveModel = old_model.into();
        let updated = web_user::ActiveModel {
            id: old_model.id,
            username: Set(model.username),
            user_id: old_model.user_id,
            access_token: Set(model.access_token),
            last_refresh: Set(model.last_refresh),
            password: Set(model.password),
            email: Set(model.email),
            last_login_ip: Set(model.last_login_ip),
            twitch_broadcaster_id: Set(model.twitch_broadcaster_id),
            create_date: old_model.create_date,
            modify_date: Set(model.modify_date),
        }
        .update(db)
        .await?;

        Ok(updated)
    } else {
        create(db, model).await
    }
}
