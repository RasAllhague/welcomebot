use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Image::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Image::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Image::OriginalName)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Image::ServerName).string_len(255).not_null())
                    .col(ColumnDef::new(Image::Path).string_len(255).not_null())
                    .col(ColumnDef::new(Image::Width).integer().not_null())
                    .col(ColumnDef::new(Image::Height).integer().not_null())
                    .col(ColumnDef::new(Image::Size).big_integer().not_null())
                    .col(ColumnDef::new(Image::CreateUserId).big_integer().not_null())
                    .col(ColumnDef::new(Image::CreateDate).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guild::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Guild::Name).string_len(50).not_null())
                    .col(ColumnDef::new(Guild::GuildId).big_integer().not_null())
                    .col(ColumnDef::new(Guild::WelcomeSettingsId).integer().null())
                    .col(ColumnDef::new(Guild::CreateUserId).big_integer().not_null())
                    .col(ColumnDef::new(Guild::CreateDate).timestamp().not_null())
                    .col(ColumnDef::new(Guild::ModifyUserId).big_integer().null())
                    .col(ColumnDef::new(Guild::ModifyDate).timestamp().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WelcomeSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WelcomeSettings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WelcomeSettings::WelcomeChannel)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WelcomeSettings::ChatMessage)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WelcomeSettings::ImageHeadline)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WelcomeSettings::ImageSubtext)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WelcomeSettings::BackBanner)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(WelcomeSettings::FrontBanner)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WelcomeSettings::Table, WelcomeSettings::BackBanner)
                            .to(Image::Table, Image::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(WelcomeSettings::Table, WelcomeSettings::FrontBanner)
                            .to(Image::Table, Image::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(Guild::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Image::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(WelcomeSettings::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    Name,
    GuildId,
    WelcomeSettingsId,
    CreateUserId,
    CreateDate,
    ModifyUserId,
    ModifyDate,
}

#[derive(DeriveIden)]
enum Image {
    Table,
    Id,
    OriginalName,
    ServerName,
    Path,
    Width,
    Height,
    Size,
    CreateUserId,
    CreateDate,
}

#[derive(DeriveIden)]
enum WelcomeSettings {
    Table,
    Id,
    WelcomeChannel,
    ChatMessage,
    ImageHeadline,
    ImageSubtext,
    FrontBanner,
    BackBanner,
}
