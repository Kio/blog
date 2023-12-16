pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_posts;
mod m20231216_011219_rename_posts_to_post;
mod m20231216_021716_post_excerpt;
mod m20231216_163732_tags;
mod m20231216_202236_tag_slug_index;
mod m20231216_202618_post_tag_relation;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_posts::Migration),
            Box::new(m20231216_011219_rename_posts_to_post::Migration),
            Box::new(m20231216_021716_post_excerpt::Migration),
            Box::new(m20231216_163732_tags::Migration),
            Box::new(m20231216_202236_tag_slug_index::Migration),
            Box::new(m20231216_202618_post_tag_relation::Migration),
        ]
    }
}
