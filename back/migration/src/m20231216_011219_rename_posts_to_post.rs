use sea_orm_migration::prelude::*;
use sea_query::{Alias, Table};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.rename_table(
            Table::rename()
                .table(Alias::new("posts"), Alias::new("post"))
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.rename_table(
            Table::rename()
                .table(Alias::new("post"), Alias::new("posts"))
                .to_owned(),
        ).await
    }
}
