use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TwitchToken::Table)
                    .if_not_exists()
                    .col(pk_auto(TwitchToken::Id))
                    .col(string_null(TwitchToken::AccessToken))
                    .col(string_null(TwitchToken::RefreshToken))
                    .col(timestamp_null(TwitchToken::LastRefreshed))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TwitchToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TwitchToken {
    Table,
    Id,
    AccessToken,
    RefreshToken,
    LastRefreshed,
}
