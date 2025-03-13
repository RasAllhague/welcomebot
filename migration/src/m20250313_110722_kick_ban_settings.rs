use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guild::Table)
                    .add_column(
                        enumeration(
                            Guild::PunishMode,
                            Alias::new("punish_mode"),
                            PunishMode::iter(),
                        )
                        .default("kick"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Guild::Table)
                    .drop_column(Guild::PunishMode)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    PunishMode,
}

#[derive(Iden, EnumIter, Default)]
pub enum PunishMode {
    #[default]
    #[iden = "kick"]
    Kick,
    #[iden = "ban"]
    Ban,
}
