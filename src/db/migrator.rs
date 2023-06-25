mod m20230619_000001_create_metadatas_table;

use sea_orm::Database;
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230619_000001_create_metadatas_table::Migration)]
    }
}

pub async fn migration(database_url: &str) -> Result<(), DbErr> {
    // let connect_options = ConnectOptions::new("mysql://admin:admin@localhost:3307".into())
    //     .set_schema_search_path("my_schema".into())
    //     .to_owned();
    let db = Database::connect(database_url).await?;
    // let schema_manager = SchemaManager::new(db);

    Migrator::up(&db, None).await?;

    Ok(())
}
