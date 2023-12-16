use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostTag::PostId).integer().not_null())
                    .col(ColumnDef::new(PostTag::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("post_tag_post_id")
                            .from(PostTag::Table, PostTag::PostId)
                            .to(Post::Table, Post::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("post_tag_tag_id")
                            .from(PostTag::Table, PostTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(PostTag::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PostTag {
    Table,
    Id,
    PostId,
    TagId,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
}
