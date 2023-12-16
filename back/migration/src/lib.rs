pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_posts;
mod m20231216_011219_rename_posts_to_post;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_posts::Migration),
            Box::new(m20231216_011219_rename_posts_to_post::Migration),
        ]
    }
}
