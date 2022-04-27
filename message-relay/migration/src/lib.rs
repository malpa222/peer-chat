pub use sea_schema::migration::prelude::*;

mod m20220424_000000_create_table;
mod m20220424_213700_create_table_message;
mod m20220424_213500_create_table_user;
mod m20220427_190400_create_table_chat;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220424_000000_create_table::Migration),
            Box::new(m20220424_213700_create_table_message::Migration),
            Box::new(m20220424_213500_create_table_user::Migration),
            Box::new(m20220427_190400_create_table_chat::Migration),
        ]
    }
}
