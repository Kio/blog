use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .table(Tag::Table)
                    .name("tag_slug")
                    .col(Tag::Slug)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .table(Tag::Table)
                    .name("tag_slug")
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Slug,
}
