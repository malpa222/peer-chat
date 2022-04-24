use sea_schema::migration::prelude::*;
use sea_query::ColumnDef;
use entity::{conversation, user_conversation};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220427_190400_create_table_conversation"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(sea_query::Table::create()
            .table(conversation::Entity)
            .if_not_exists()
            .col(ColumnDef::new(conversation::Column::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(conversation::Column::CreatedAt).timestamp().not_null())
            .col(ColumnDef::new(conversation::Column::UpdatedAt).timestamp().not_null())
            .to_owned()).await?;

        manager.create_table(sea_query::Table::create()
            .table(user_conversation::Entity)
            .if_not_exists()
            .col(ColumnDef::new(user_conversation::Column::Id).uuid().not_null().primary_key())
            .col(ColumnDef::new(user_conversation::Column::UserId).uuid().not_null())
            .col(ColumnDef::new(user_conversation::Column::CreatedAt).timestamp().not_null())
            .col(ColumnDef::new(user_conversation::Column::UpdatedAt).timestamp().not_null())
            .to_owned()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(conversation::Entity)
                    .to_owned()).await?;

        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(user_conversation::Entity)
                    .to_owned()).await?;

        Ok(())
    }
}
