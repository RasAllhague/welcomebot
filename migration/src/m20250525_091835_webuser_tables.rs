use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WebUser::Table)
                    .if_not_exists()
                    .col(pk_auto(WebUser::Id))
                    .col(string_len(WebUser::Username, 50))
                    .col(big_integer_uniq(WebUser::UserId))
                    .col(string_len_null(WebUser::AccessToken, 255))
                    .col(date_time_null(WebUser::LastRefresh))
                    .col(string_len_null(WebUser::Password, 255))
                    .col(string_len_null(WebUser::Email, 255))
                    .col(string_len_null(WebUser::LastLoginIp, 50))
                    .col(integer_null(WebUser::TwitchBroadcasterId))
                    .col(date_time(WebUser::CreateDate))
                    .col(date_time_null(WebUser::ModifyDate))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WebUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WebUser {
    Table,
    Id,
    Username,
    UserId,
    AccessToken,
    LastRefresh,
    Password,
    Email,
    LastLoginIp,
    TwitchBroadcasterId,
    CreateDate,
    ModifyDate,
}
