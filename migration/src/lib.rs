pub use sea_orm_migration::prelude::*;


mod m20261112_073338_create_table_users;
mod m20261112_085046_create_table_tasks;
pub mod model;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261112_073338_create_table_users::Migration),
            Box::new(m20261112_085046_create_table_tasks::Migration),
        ]
    }
}
