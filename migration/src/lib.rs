pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240910_080929_seeding_image;
mod m20250104_223723_autoban_roles;
mod m20250119_145427_guild_auto_ban_role_id_and_log;



pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240910_080929_seeding_image::Migration),
            Box::new(m20250104_223723_autoban_roles::Migration),
            Box::new(m20250119_145427_guild_auto_ban_role_id_and_log::Migration),
        ]
    }
}
