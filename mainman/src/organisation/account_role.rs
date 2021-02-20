use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    db::Connection,
    organisation::{Organisation, OrganisationAccount},
    schema::account_role,
    MainmanResult,
};

#[derive(
    Debug, Serialize, Queryable, Associations, AsChangeset, Identifiable,
)]
#[table_name = "account_role"]
#[belongs_to(Organisation, foreign_key = "organisation")]
#[belongs_to(OrganisationAccount, foreign_key = "id")]
pub struct AccountRole {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub organisation: Option<i32>,
    pub name: String,
    pub rights: serde_json::Value,
}

impl AccountRole {
    pub fn public_role(name: &str, conn: &Connection) -> MainmanResult<Self> {
        Ok(account_role::table
            .filter(
                account_role::organisation
                    .is_null()
                    .and(account_role::name.eq(name)),
            )
            .first::<Self>(conn)?)
    }
}
