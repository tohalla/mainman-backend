use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json;
use uuid::Uuid;

use crate::appliance::Appliance;
use crate::db::Pool;
use crate::error::ApiError;
use crate::schema::{maintainer, maintainer_appliance};

pub mod handler;
pub mod routes;

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
pub struct Maintainer {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub organisation: i32,
    pub account: Option<i32>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Queryable, Associations)]
#[table_name = "maintainer_appliance"]
#[belongs_to(Maintainer, foreign_key = "maintainer")]
#[belongs_to(Appliance, foreign_key = "appliance")]
pub struct MaintainerAppliance {
    pub maintainer: i32,
    pub appliance: Uuid,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintainer"]
pub struct CreateMaintainer {
    account: Option<i32>,
    organisation: i32,
    details: serde_json::Value,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "maintainer"]
pub struct PatchMaintainer {
    id: i32,
    account: Option<i32>,
    details: Option<serde_json::Value>,
}

pub fn find(pool: &Pool, id: i32) -> Result<Maintainer, ApiError> {
    use crate::schema::maintainer::dsl;

    let conn = pool.get()?;
    let res = dsl::maintainer
        .find(id)
        .first::<Maintainer>(&conn)
        .map_err(|_| ApiError::NotFound)?;

    Ok(res)
}

pub fn get_all(
    pool: &Pool,
    organisation: i32,
) -> Result<Vec<Maintainer>, ApiError> {
    use crate::schema::maintainer::dsl;

    let conn = pool.get()?;
    let res = dsl::maintainer
        .filter(dsl::organisation.eq(organisation))
        .load::<Maintainer>(&conn)
        .map_err(|_| ApiError::NotFound)?;

    Ok(res)
}

pub fn create(
    pool: &Pool,
    payload: &CreateMaintainer,
) -> Result<Maintainer, ApiError> {
    use crate::schema::maintainer::dsl::*;

    let conn = pool.get()?;
    let res = diesel::insert_into(maintainer)
        .values(payload)
        .get_result::<Maintainer>(&conn)?;

    Ok(res)
}

pub fn patch(
    pool: &Pool,
    payload: &PatchMaintainer,
) -> Result<Maintainer, ApiError> {
    use crate::schema::maintainer::dsl::*;

    let conn = pool.get()?;
    let res = diesel::update(maintainer.find(payload.id))
        .set(payload)
        .get_result::<Maintainer>(&conn)?;

    Ok(res)
}
