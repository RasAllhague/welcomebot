use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Image::Table)
            .columns([
                Image::Id,
                Image::OriginalName,
                Image::ServerName,
                Image::Path,
                Image::Width,
                Image::Height,
                Image::Size,
                Image::CreateDate,
                Image::CreateUserId,
            ])
            .values_panic([
                1.into(),
                "default_userbanner_back.png".into(),
                "default_userbanner_back.png".into(),
                "assets/images/default_userbanner_back.png".into(),
                900.into(),
                400.into(),
                2780.into(),
                "2024-08-20 09:24:15.421242300".into(),
                "1236977267222249512".into(),
            ])
            .values_panic([
                2.into(),
                "default_userbanner.png".into(),
                "default_userbanner.png".into(),
                "assets/images/default_userbanner.png".into(),
                900.into(),
                400.into(),
                268474.into(),
                "2024-08-20 09:24:15.421242300".into(),
                "1236977267222249512".into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
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
