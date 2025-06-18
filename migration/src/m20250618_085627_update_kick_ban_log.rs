use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KickEntry::Table)
                    .if_not_exists()
                    .col(pk_auto(KickEntry::Id))
                    .col(big_integer(KickEntry::UserId))
                    .col(string_len(KickEntry::UserName, 50))
                    .col(string_len_null(KickEntry::Reason, 500))
                    .col(integer(KickEntry::GuildId))
                    .col(big_integer(KickEntry::CreateUserId))
                    .col(timestamp(KickEntry::CreateDate))
                    .foreign_key(
                        ForeignKey::create()
                            .from(KickEntry::Table, KickEntry::GuildId)
                            .to(Guild::Table, Guild::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KickEntry::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum KickEntry {
    Table,
    Id,
    UserId,
    UserName,
    Reason,
    GuildId,
    CreateUserId,
    CreateDate,
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
}
