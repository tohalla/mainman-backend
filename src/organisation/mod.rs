use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::account::Account;
use crate::db::Pool;
use crate::error::ApiError;
use crate::schema::{organisation, organisation_account};

pub mod handler;
pub mod routes;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
pub struct Organisation {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
    pub organisation_identifier: Option<String>,
    pub locale: String,
    pub admin_account: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "organisation_account"]
#[belongs_to(Account, foreign_key = "account")]
#[belongs_to(Organisation, foreign_key = "organisation")]
pub struct OrganisationAccount {
    pub id: i32,
    pub organisation: i32,
    pub account: i32,
    pub account_role: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "organisation"]
pub struct CreateOrganisation<'a> {
    name: &'a str,
    organisation_identifier: &'a str,
    locale: &'a str,
    admin_account: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "organisation"]
pub struct PatchOrganisation {
    id: i32,
    name: Option<String>,
    organisation_identifier: Option<String>,
    locale: Option<String>,
}

pub fn find(
    pool: &Pool,
    organisation_id: i32,
) -> Result<Organisation, ApiError> {
    use crate::schema::organisation::dsl::*;

    let conn = pool.get()?;
    let res = organisation
        .find(organisation_id)
        .first::<Organisation>(&conn)
        .map_err(|_| ApiError::NotFound)?;

    Ok(res)
}

pub fn get_all(
    pool: &Pool,
    account_id: i32,
) -> Result<Vec<Organisation>, ApiError> {
    use crate::schema::organisation::dsl::*;

    let conn = pool.get()?;
    let res = organisation
        .filter(admin_account.eq(account_id))
        .load::<Organisation>(&conn)
        .map_err(|_| ApiError::NotFound)?;

    Ok(res)
}

pub fn create(
    pool: &Pool,
    payload: CreateOrganisation,
) -> Result<Organisation, ApiError> {
    use crate::schema::organisation::dsl::*;

    let conn = pool.get()?;
    let res = diesel::insert_into(organisation)
        .values(payload)
        .get_result::<Organisation>(&conn)?;

    Ok(res)
}

pub fn patch(
    pool: &Pool,
    payload: &PatchOrganisation,
) -> Result<Organisation, ApiError> {
    use crate::schema::organisation::dsl::*;

    let conn = pool.get()?;
    let res = diesel::update(organisation.find(payload.id))
        .set(payload)
        .get_result::<Organisation>(&conn)?;

    Ok(res)
}
