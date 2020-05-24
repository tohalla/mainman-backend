use crate::schema::account;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "account"]
pub struct Account {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub password: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "account"]
pub struct CreateAccountPayload {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "account"]
pub struct UpdateAccountPayload {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
