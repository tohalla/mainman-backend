use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::db::Pool;
use crate::error::ApiError;
use crate::schema::account;

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
    pub password: Vec<u8>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "account"]
pub struct CreateAccount<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
    password: &'a [u8],
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "account"]
pub struct UpdateAccount<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
}

pub fn find(
    pool: &Pool,
    account_id: i32,
) -> Result<handler::AccountResponse, ApiError> {
    use crate::schema::account::dsl::*;

    let conn = pool.get()?;
    let account_response = account
        .find(account_id)
        .first::<Account>(&conn)
        .map_err(|_| ApiError::NotFound)?;

    Ok(account_response.into())
}

pub fn create(
    pool: &Pool,
    new_account: CreateAccount,
) -> Result<handler::AccountResponse, ApiError> {
    use crate::schema::account::dsl::*;

    let conn = pool.get()?;
    let res = diesel::insert_into(account)
        .values(new_account)
        .get_result::<Account>(&conn)?;

    Ok(res.into())
}
