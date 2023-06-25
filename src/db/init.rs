use crate::db::migrator::Migrator;
use sea_orm::{ConnectOptions, Database, DbErr};
use sea_orm_migration::prelude::*;

pub async fn init(database_url: &str) -> Result<(), DbErr> {
    // let connect_options = ConnectOptions::new("mysql://admin:admin@localhost:3307".into())
    //     .set_schema_search_path("my_schema".into())
    //     .to_owned();
    let db = Database::connect(database_url).await?;
    // let schema_manager = SchemaManager::new(db);

    Migrator::up(&db, None).await?;

    Ok(())
}
