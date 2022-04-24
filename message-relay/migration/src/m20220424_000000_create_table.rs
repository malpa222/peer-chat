use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220424_213500_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        return Ok(());
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        return Ok(());
    }
}
