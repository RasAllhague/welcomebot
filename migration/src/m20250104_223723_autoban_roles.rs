use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AutoBanRole::Table)
                    .if_not_exists()
                    .col(pk_auto(AutoBanRole::Id))
                    .col(big_integer_uniq(AutoBanRole::RoleId))
                    .col(integer(AutoBanRole::GuildId))
                    .col(big_integer(AutoBanRole::CreateUserId))
                    .col(date_time(AutoBanRole::CreateDate))
                    .foreign_key(
                        ForeignKey::create()
                            .from(AutoBanRole::Table, AutoBanRole::GuildId)
                            .to(Guild::Table, Guild::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AutoBanRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AutoBanRole {
    Table,
    Id,
    RoleId,
    GuildId,
    CreateUserId,
    CreateDate,
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
}
