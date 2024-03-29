pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20240212_003118_create_session_table;
mod m20240214_180047_create_profile_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20240212_003118_create_session_table::Migration),
            Box::new(m20240214_180047_create_profile_table::Migration),
        ]
    }
}
