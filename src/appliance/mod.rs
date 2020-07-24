use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::Pool;
use crate::error::ApiError;
use crate::schema::appliance;

pub mod handler;
pub mod routes;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Appliance {
    pub hash: uuid::Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
    pub description: Option<String>,
    pub organisation: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "appliance"]
pub struct CreateAppliance {
    name: String,
    description: Option<String>,
    organisation: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "appliance"]
pub struct PatchAppliance {
    hash: uuid::Uuid,
    name: Option<String>,
    description: Option<String>,
}

pub fn find(pool: &Pool, hash: Uuid) -> Result<Appliance, ApiError> {
    use crate::schema::appliance::dsl;

    let conn = pool.get()?;
    let res = dsl::appliance
        .find(hash)
        .first::<Appliance>(&conn)
        .map_err(|_| ApiError::NotFound)?;

    Ok(res)
}

pub fn get_all(
    pool: &Pool,
    organisation: i32,
) -> Result<Vec<Appliance>, ApiError> {
    use crate::schema::appliance::dsl;

    let conn = pool.get()?;
    let res = dsl::appliance
        .filter(dsl::organisation.eq(organisation))
        .load::<Appliance>(&conn)
        .map_err(|_| ApiError::NotFound)?;

    Ok(res)
}

pub fn create(
    pool: &Pool,
    payload: CreateAppliance,
) -> Result<Appliance, ApiError> {
    use crate::schema::appliance::dsl::*;

    let conn = pool.get()?;
    let res = diesel::insert_into(appliance)
        .values(payload)
        .get_result::<Appliance>(&conn)?;

    Ok(res)
}

pub fn patch(
    pool: &Pool,
    payload: &PatchAppliance,
) -> Result<Appliance, ApiError> {
    use crate::schema::appliance::dsl::*;

    let conn = pool.get()?;
    let res = diesel::update(appliance.find(payload.hash))
        .set(payload)
        .get_result::<Appliance>(&conn)?;

    Ok(res)
}
