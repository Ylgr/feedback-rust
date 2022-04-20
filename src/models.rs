use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};
use serde::{Serialize, Deserialize};
use crate::schema::feedback;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "feedback"]
pub struct Feedback {
    pub title: String,
    pub content: String,
}

#[derive(Queryable, Serialize)]
pub struct FeedbackQuery {
    pub index: i32,
    pub title: String,
    pub content: String,
}


impl Feedback {
    pub fn get_feeds(conn: &PgConnection) -> Vec<FeedbackQuery> {
        use crate::schema::feedback::dsl::*;
        feedback
            .load::<FeedbackQuery>(conn)
            .unwrap()
    }

    pub fn new_feed(feedback: Feedback,  conn: &PgConnection) -> bool {
        diesel::insert_into(crate::schema::feedback::table)
            .values(&feedback)
            .execute(conn)
            .is_ok()
    }
}