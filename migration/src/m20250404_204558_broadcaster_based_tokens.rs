use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TwitchToken::Table).to_owned())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(TwitchBroadcaster::Table)
                    .if_not_exists()
                    .col(pk_auto(TwitchBroadcaster::Id))
                    .col(string(TwitchBroadcaster::BroadcasterLogin))
                    .col(string(TwitchBroadcaster::BroadcasterId).unique_key())
                    .col(string(TwitchBroadcaster::BroadcasterName))
                    .col(string(TwitchBroadcaster::AccessToken))
                    .col(string_null(TwitchBroadcaster::RefreshToken))
                    .col(timestamp_null(TwitchBroadcaster::LastRefreshed))
                    .col(timestamp(TwitchBroadcaster::CreateDate))
                    .col(timestamp_null(TwitchBroadcaster::ModifyDate))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TwitchBroadcaster::Table).to_owned())
            .await?;
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
}

#[derive(DeriveIden)]
enum TwitchToken {
    Table,
    Id,
    AccessToken,
    RefreshToken,
    LastRefreshed,
}

#[derive(DeriveIden)]
enum TwitchBroadcaster {
    Table,
    Id,
    BroadcasterLogin,
    BroadcasterId,
    BroadcasterName,
    AccessToken,
    RefreshToken,
    LastRefreshed,
    CreateDate,
    ModifyDate,
}
