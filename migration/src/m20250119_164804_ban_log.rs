use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guild::Table)
                    .add_column_if_not_exists(big_integer_null(Guild::ModerationChannelId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(BanEntry::Table)
                    .if_not_exists()
                    .col(pk_auto(BanEntry::Id))
                    .col(big_integer(BanEntry::UserId))
                    .col(string_len(BanEntry::UserName, 50))
                    .col(string_len_null(BanEntry::Reason, 500))
                    .col(integer(BanEntry::GuildId))
                    .col(big_integer(BanEntry::CreateUserId))
                    .col(timestamp(BanEntry::CreateDate))
                    .foreign_key(
                        ForeignKey::create()
                            .from(BanEntry::Table, BanEntry::GuildId)
                            .to(Guild::Table, Guild::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BanEntry::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BanEntry {
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
    ModerationChannelId,
}
