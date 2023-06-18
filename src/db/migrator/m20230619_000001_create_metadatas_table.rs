use crate::db::metadatas::Metadatas;
use sea_orm::sea_query::{ColumnDef, Table};
use sea_orm::DbErr;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230619_000001_create_metadatas_table.rs"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Metadatas::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Metadatas::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Metadatas::ContractAddress)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Metadatas::TokenId).integer())
                    .col(ColumnDef::new(Metadatas::Metadata).string())
                    .col(ColumnDef::new(Metadatas::CreatedAt).timestamp())
                    .col(ColumnDef::new(Metadatas::UpdatedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Metadatas::Table).to_owned())
            .await
    }
}
