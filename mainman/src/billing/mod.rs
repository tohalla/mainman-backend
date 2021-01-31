use chrono::NaiveDateTime;

use crate::schema::card;

pub mod handler;
pub mod routes;

#[derive(Debug, Serialize, Queryable, Associations, Identifiable)]
#[table_name = "card"]
struct Card {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub account: i32,
}
