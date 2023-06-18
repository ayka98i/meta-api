#[macro_use]
extern crate rocket;

use chrono::{DateTime, Local};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

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
fn get_contracts() -> Json<Test> {
    Json(Test::new())
}

#[post("/")]
fn add_contracts() {}

#[delete("/")]
fn delete_contracts() {}

#[get("/<contract_address>/tokens")]
fn get_tokens_metadata(contract_address: &str) {}

#[post("/<contract_address>/tokens")]
fn add_tokens_metadata(contract_address: &str) {}

#[delete("/<contract_address>/tokens")]
fn delete_tokens_metadata(contract_address: &str) {}

#[get("/<contract_address>/tokens/<token_id>")]
fn get_token_metadata(contract_address: &str, token_id: u64) {}

#[post("/<contract_address>/tokens/<token_id>")]
fn add_token_metadata(contract_address: &str, token_id: u64) {}

#[put("/<contract_address>/tokens/<token_id>")]
fn update_token_metadata(contract_address: &str, token_id: u64) {}

#[delete("/<contract_address>/tokens/<token_id>")]
fn delete_token_metadata(contract_address: &str, token_id: u64) {}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
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
