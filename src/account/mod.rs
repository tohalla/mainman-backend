use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{db::Connection, schema::account, MainmanResult};

pub mod handler;
pub mod routes;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
pub struct Account {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    #[serde(skip)]
    pub password: Vec<u8>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "account"]
pub struct NewAccount<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
    password: &'a [u8],
}

impl Account {
    pub fn get(id: i32, conn: &Connection) -> MainmanResult<Self> {
        Ok(account::dsl::account.find(id).first::<Account>(conn)?)
    }
}

impl<'a> NewAccount<'a> {
    pub fn insert(&self, conn: &Connection) -> MainmanResult<Account> {
        Ok(diesel::insert_into(account::dsl::account)
            .values(self)
            .get_result::<Account>(conn)?)
    }
}
