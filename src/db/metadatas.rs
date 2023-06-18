use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Metadatas {
    Table,
    Id,
    ContractAddress,
    TokenId,
    Metadata,
    CreatedAt,
    UpdatedAt,
}
