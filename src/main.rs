#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;

use rocket::{get, post, routes};
// use rocket_contrib::json::Json;
use std::env;
use dotenv::dotenv;
use crate::models::Feedback;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



// #[post("/feedback", format = "json", data = "<feed>")]
// fn send_feedback(feed: Json<models::Feed>) {
//
// }

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount("/", routes![hello])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
