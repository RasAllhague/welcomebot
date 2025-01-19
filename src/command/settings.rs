use chrono::Utc;
use entity::guild;
use log::error;
use migration::sea_orm::DbConn;
use poise::serenity_prelude::{self as serenity, CreateMessage, Guild};
use welcome_service::{
    guild_mutation, guild_query, welcome_settings_mutation, welcome_settings_query,
};

use crate::Data;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// Settings of welcome bot. With this you can update its behaviour.
#[poise::command(
    slash_command,
    guild_only,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn settings(
    ctx: Context<'_>,
    #[description = "The text of the chat welcome message. Placeholders: {user}, {guild_name}"]
    chat_message: Option<String>,
    #[description = "The text of the healine of the image. Placeholders: {name}"]
    image_headline: Option<String>,
    #[description = "The text of the subline of the image. Placeholders: {members}"] image_subline: Option<String>,
    #[description = "The channel where to send welcome messages to"]
    #[channel_types("Text")]
    channel: Option<serenity::Channel>,
    #[description = "A role which should be automatic banned if a user has aquired this role"]
    autoban_role: Option<serenity::RoleId>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    // unwrap since we are in a guild only command
    let guild = ctx.guild().unwrap().clone();
    let author_id = ctx.author().id.into();

    if let Err(why) = update_welcome_settings(
        db,
        &guild,
        author_id,
        chat_message,
        image_headline,
        image_subline,
        channel.map(|x| x.id()).or(guild.system_channel_id),
    )
    .await
    {
        error!("Could not update welcome channel: {why}");

        ctx.channel_id()
            .send_message(
                &ctx,
                CreateMessage::new().content(format!("Could not update welcome channel.")),
            )
            .await?;
    }

    if let Some(role_id) = autoban_role {
        if let Some(mut guild) = guild_query::get_by_guild_id(db, guild.id.into()).await? {
            guild.auto_ban_role_id = Some(role_id.into());
            guild_mutation::update(db, guild).await?;
        }
        else {
            let guild = guild::Model {
                id: 0,
                name: guild.name.clone(),
                guild_id: guild.id.into(),
                welcome_settings_id: None,
                auto_ban_role_id: Some(role_id.into()),
                create_user_id: author_id,
                create_date: Utc::now().naive_utc().to_string(),
                modify_date: None,
                modify_user_id: None,
            };

            guild_mutation::create(db, guild).await?;
        }
    }

    ctx.say("Finished updating.").await?;

    Ok(())
}

async fn update_welcome_settings(
    db: &DbConn,
    discord_guild: &Guild,
    create_user_id: i64,
    chat_message: Option<String>,
    image_headline: Option<String>,
    image_subline: Option<String>,
    channel: Option<serenity::ChannelId>,
) -> Result<(), PoiseError> {
    let guild_id = discord_guild.id.into();

    if let Some(mut guild) = guild_query::get_by_guild_id(db, guild_id).await? {
        if let Some(mut welcome_settings) = welcome_settings_query::get_one(db, guild.id).await? {
            welcome_settings.welcome_channel = match channel {
                Some(c) => c.into(),
                None => welcome_settings.welcome_channel,
            };
            welcome_settings.chat_message = chat_message.unwrap_or(welcome_settings.chat_message);
            welcome_settings.image_headline =
                image_headline.unwrap_or(welcome_settings.image_headline);
            welcome_settings.image_subtext =
                image_subline.unwrap_or(welcome_settings.image_subtext);

            welcome_settings_mutation::update(db, welcome_settings).await?;
        } else {
            let welcome_settings = entity::welcome_settings::Model {
                id: 0,
                welcome_channel: 0,
                chat_message: chat_message
                    .unwrap_or("Hey {user}, welcome to **{guild_name}**".to_string()),
                image_headline: image_headline
                    .unwrap_or("{name} just joined the server".to_string()),
                image_subtext: image_subline.unwrap_or("You are the #{members} member".to_string()),
                back_banner: 1,
                front_banner: 2,
                create_user_id: create_user_id,
                create_date: Utc::now().naive_utc().to_string(),
                modify_date: None,
                modify_user_id: None,
            };

            let welcome_settings = welcome_settings_mutation::create(db, welcome_settings).await?;
            guild.welcome_settings_id = Some(welcome_settings.id);

            guild_mutation::update(db, guild).await?;
        }
    } else {
        let welcome_settings = entity::welcome_settings::Model {
            id: 0,
            welcome_channel: 0,
            chat_message: chat_message
                .unwrap_or("Hey {user}, welcome to **{guild_name}**".to_string()),
            image_headline: image_headline.unwrap_or("{name} just joined the server".to_string()),
            image_subtext: image_subline.unwrap_or("You are the #{members} member".to_string()),
            back_banner: 1,
            front_banner: 2,
            create_user_id: create_user_id,
            create_date: Utc::now().naive_utc().to_string(),
            modify_date: None,
            modify_user_id: None,
        };

        let welcome_settings = welcome_settings_mutation::create(db, welcome_settings).await?;

        let guild = guild::Model {
            id: 0,
            name: discord_guild.name.clone(),
            guild_id,
            welcome_settings_id: Some(welcome_settings.id),
            auto_ban_role_id: None,
            create_user_id: create_user_id,
            create_date: Utc::now().naive_utc().to_string(),
            modify_date: None,
            modify_user_id: None,
        };

        guild_mutation::create(db, guild).await?;
    }

    Ok(())
}
