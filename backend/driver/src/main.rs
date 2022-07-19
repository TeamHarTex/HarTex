#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[rocket::main]
pub async fn main() {
    rocket::build().launch().await.unwrap();
}
