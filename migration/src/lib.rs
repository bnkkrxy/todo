pub use sea_orm_migration::prelude::*;

mod m20260303_183523_create_category;
mod m20260303_185427_create_task;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260303_183523_create_category::Migration),
            Box::new(m20260303_185427_create_task::Migration),
        ]
    }
}
