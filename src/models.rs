use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::Serialize;
use crate::schema::feedback;

#[derive(Queryable, Insertable, Serialize)]
#[table_name = "feedback"]
pub struct Feedback {
    pub title: String,
    pub content: String,
}

impl Feedback {
    pub fn new_feed(_title: String, _content: String, conn: &PgConnection) -> bool {
        diesel::insert_into(crate::schema::feedback::table)
            .values(& Feedback { title: _title, content: _content})
            .execute(conn)
            .is_ok()
    }
}