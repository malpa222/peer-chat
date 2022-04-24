use sea_schema::migration::prelude::*;
use sea_query::ColumnDef;
use entity::user;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220424_213500_create_table_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(sea_query::Table::create()
            .table(user::Entity)
            .if_not_exists()
            .col(ColumnDef::new(user::Column::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(user::Column::Email).string().not_null().unique_key())
            .col(ColumnDef::new(user::Column::CreatedAt).timestamp().not_null())
            .col(ColumnDef::new(user::Column::UpdatedAt).timestamp().not_null())
            .to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(user::Entity)
                    .to_owned()).await
    }
}
