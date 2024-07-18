pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_tags;
mod m20240718_144412_create_posts;
mod m20240718_150817_create_posts_tags;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_tags::Migration),
            Box::new(m20240718_144412_create_posts::Migration),
            Box::new(m20240718_150817_create_posts_tags::Migration),
        ]
    }
}
