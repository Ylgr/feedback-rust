#[macro_use]
extern crate diesel;

mod models;
mod schema;

use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::{get, post, routes};
use std::env;
use dotenv::dotenv;
use crate::models::{Feedback, FeedbackQuery};

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


#[get("/feedback")]
fn get_feedback() -> Json<Vec<FeedbackQuery>> {
    let conn = establish_connection();
    let feeds = Feedback::get_feeds(&conn);
    Json(feeds)
}

#[post("/feedback", data = "<feedback>")]
fn send_feedback(feedback: Json<Feedback>) -> Json<bool> {
    println!("Check content: ");
    println!("{}", feedback.title);
    println!("{}", feedback.content);
    let conn = establish_connection();
    let new_status = Feedback::new_feed(feedback.into_inner(), &conn);
    println!("Updating: {}", new_status);
    Json(new_status)
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    rocket::build()
        .mount("/", routes![hello, send_feedback, get_feedback])
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
