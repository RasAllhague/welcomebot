use ::entity::guild::{self, Entity as Guild};
use chrono::Utc;

use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, Set};

/// Creates a new gamekey.
///
/// # Errors
///
/// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
#[fastrace::trace]
pub async fn create(db: &DbConn, guild: guild::Model) -> Result<guild::Model, DbErr> {
    guild::ActiveModel {
        name: Set(guild.name),
        guild_id: Set(guild.guild_id),
        moderation_channel_id: Set(guild.moderation_channel_id),
        welcome_settings_id: Set(guild.welcome_settings_id),
        auto_ban_role_id: Set(guild.auto_ban_role_id),
        ban_reason_template: Set(guild.ban_reason_template),
        create_user_id: Set(guild.create_user_id),
        create_date: Set(guild.create_date),
        ..Default::default()
    }
    .insert(db)
    .await
}

#[fastrace::trace]
pub async fn get_or_create<T: AsRef<str> + std::marker::Send>(
    db: &DbConn,
    guild_id: i64,
    guild_name: T,
    create_user_id: i64,
) -> Result<guild::Model, DbErr> {
    if let Some(g) = crate::query::guild::get_by_guild_id(db, guild_id).await? {
        Ok(g)
    } else {
        let guild = guild::Model {
            id: 0,
            name: guild_name.as_ref().to_string(),
            guild_id,
            welcome_settings_id: None,
            moderation_channel_id: None,
            auto_ban_role_id: None,
            ban_reason_template: None,
            punish_mode: "kick".to_string(),
            create_user_id,
            create_date: Utc::now().naive_utc().to_string(),
            modify_date: None,
            modify_user_id: None,
        };

        create(db, guild).await
    }
}

/// Updates the details of a gamekey.
///
/// # Errors
///
/// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
#[fastrace::trace]
pub async fn update(
    db: &DbConn,
    update_guild: &guild::Model,
) -> Result<Option<guild::Model>, DbErr> {
    let guild: guild::ActiveModel = match Guild::find_by_id(update_guild.id).one(db).await? {
        Some(m) => m.into(),
        None => return Ok(None),
    };

    let updated = guild::ActiveModel {
        id: guild.id,
        name: Set(update_guild.name.clone()),
        guild_id: Set(update_guild.guild_id),
        moderation_channel_id: Set(update_guild.moderation_channel_id),
        welcome_settings_id: Set(update_guild.welcome_settings_id),
        auto_ban_role_id: Set(update_guild.auto_ban_role_id),
        punish_mode: Set(update_guild.punish_mode.clone()),
        ban_reason_template: Set(update_guild.ban_reason_template.clone()),
        create_date: guild.create_date,
        create_user_id: guild.create_user_id,
        modify_date: Set(update_guild.modify_date.clone()),
        modify_user_id: Set(update_guild.modify_user_id),
    }
    .update(db)
    .await?;

    Ok(Some(updated))
}
