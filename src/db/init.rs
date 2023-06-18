use sea_orm::{Database, DbErr};
use sea_orm_migration::prelude::*;

pub async fn init(database_url: &str) -> Result<(), DbErr> {
    // let db = Database::connect("mysql://admin:admin@localhost:3307").await?;
    todo!()
}
