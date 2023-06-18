#[macro_use]
extern crate rocket;

use chrono::{DateTime, Local};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use sea_orm::{Database, DatabaseConnection};

#[get("/")]
fn get_health() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Test {
    status_code: u16,
    unix_ts: i64,
}

impl Test {
    fn new() -> Test {
        Test {
            status_code: Status::Ok.code,
            unix_ts: Local::now().timestamp(),
        }
    }
}

#[get("/")]
fn get_contracts() {
    todo!();
}

#[post("/")]
fn add_contracts() {
    todo!();
}

#[delete("/")]
fn delete_contracts() {
    todo!();
}

#[get("/<contract_address>/tokens")]
fn get_tokens_metadata(contract_address: &str) {
    todo!();
}

#[post("/<contract_address>/tokens")]
fn add_tokens_metadata(contract_address: &str) {
    todo!();
}

#[delete("/<contract_address>/tokens")]
fn delete_tokens_metadata(contract_address: &str) {
    todo!();
}

#[get("/<contract_address>/tokens/<token_id>")]
fn get_token_metadata(contract_address: &str, token_id: u64) {
    todo!();
}

#[post("/<contract_address>/tokens/<token_id>")]
fn add_token_metadata(contract_address: &str, token_id: u64) {
    todo!();
}

#[put("/<contract_address>/tokens/<token_id>")]
fn update_token_metadata(contract_address: &str, token_id: u64) {
    todo!();
}

#[delete("/<contract_address>/tokens/<token_id>")]
fn delete_token_metadata(contract_address: &str, token_id: u64) {
    todo!();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let db: DatabaseConnection = Database::connect("mysql://admin:admin@localhost:3307")
        .await
        .unwrap();

    let _rocket = rocket::build()
        .mount("/health", routes![get_health])
        .mount(
            "/v1/contracts",
            routes![
                get_contracts,
                add_contracts,
                delete_contracts,
                get_tokens_metadata,
                add_tokens_metadata,
                delete_tokens_metadata,
                get_token_metadata,
                add_token_metadata,
                update_token_metadata,
                delete_token_metadata
            ],
        )
        .launch()
        .await?;

    Ok(())
}
