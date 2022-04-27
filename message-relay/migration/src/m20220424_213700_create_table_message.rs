use sea_schema::migration::prelude::*;
use sea_query::ColumnDef;
use entity::message;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220424_213500_create_table_message"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(sea_query::Table::create()
            .table(message::Entity)
            .if_not_exists()
            .col(ColumnDef::new(message::Column::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(message::Column::UserId).uuid().not_null())
            .col(ColumnDef::new(message::Column::ChatId).uuid().not_null())
            .col(ColumnDef::new(message::Column::Payload).string().not_null())
            .col(ColumnDef::new(message::Column::CreatedAt).timestamp().not_null())
            .col(ColumnDef::new(message::Column::UpdatedAt).timestamp().not_null())
            .to_owned())
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(message::Entity)
                    .to_owned()).await
    }
}
