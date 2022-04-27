use sea_schema::migration::prelude::*;
use sea_query::ColumnDef;
use entity::{chat, user_chat};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220427_190400_create_table_chat"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(sea_query::Table::create()
            .table(chat::Entity)
            .if_not_exists()
            .col(ColumnDef::new(chat::Column::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(chat::Column::CreatedAt).timestamp().not_null())
            .col(ColumnDef::new(chat::Column::UpdatedAt).timestamp().not_null())
            .to_owned()).await?;

        manager.create_table(sea_query::Table::create()
            .table(user_chat::Entity)
            .if_not_exists()
            .col(ColumnDef::new(user_chat::Column::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(user_chat::Column::UserId).uuid().not_null())
            .col(ColumnDef::new(user_chat::Column::CreatedAt).timestamp().not_null())
            .col(ColumnDef::new(user_chat::Column::UpdatedAt).timestamp().not_null())
            .to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(chat::Entity)
                    .to_owned()).await?;

        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(user_chat::Entity)
                    .to_owned()).await?;

        Ok(())
    }
}
