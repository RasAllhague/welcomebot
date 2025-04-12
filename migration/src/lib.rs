pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240910_080929_seeding_image;
mod m20250104_223723_autoban_roles;
mod m20250119_145427_guild_auto_ban_role_id_and_log;
mod m20250119_164804_ban_log;
mod m20250121_074701_ban_message_text;
mod m20250121_102948_enable_welcome_message;
mod m20250313_110722_kick_ban_settings;
mod m20250327_133628_twitch_tables;
mod m20250404_204558_broadcaster_based_tokens;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240910_080929_seeding_image::Migration),
            Box::new(m20250104_223723_autoban_roles::Migration),
            Box::new(m20250119_145427_guild_auto_ban_role_id_and_log::Migration),
            Box::new(m20250119_164804_ban_log::Migration),
            Box::new(m20250121_074701_ban_message_text::Migration),
            Box::new(m20250121_102948_enable_welcome_message::Migration),
            Box::new(m20250313_110722_kick_ban_settings::Migration),
            Box::new(m20250327_133628_twitch_tables::Migration),
            Box::new(m20250404_204558_broadcaster_based_tokens::Migration),
        ]
    }
}
