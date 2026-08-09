pub use sea_orm_migration::prelude::*;

mod m20260630_084121_init;
mod m20260722_093719_ai_models;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260630_084121_init::Migration),
            Box::new(m20260722_093719_ai_models::Migration),
        ]
    }
}
