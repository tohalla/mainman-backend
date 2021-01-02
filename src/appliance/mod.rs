use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{db::Pool, error::Error, schema::appliance, MainmanResult};

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
pub struct CreateAppliance<'a> {
    name: &'a str,
    description: &'a str,
    organisation: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "appliance"]
pub struct PatchAppliance {
    hash: uuid::Uuid,
    name: Option<String>,
    description: Option<String>,
}

pub fn find(pool: &Pool, hash: Uuid) -> MainmanResult<Appliance> {
    use crate::schema::appliance::dsl;
    Ok(dsl::appliance.find(hash).first::<Appliance>(&pool.get()?)?)
}

pub fn get_all(
    pool: &Pool,
    organisation: i32,
) -> MainmanResult<Vec<Appliance>> {
    use crate::schema::appliance::dsl;

    let conn = pool.get()?;
    let res = dsl::appliance
        .filter(dsl::organisation.eq(organisation))
        .load::<Appliance>(&conn)
        .map_err(|_| Error::NotFoundError)?;

    Ok(res)
}

pub fn create(
    pool: &Pool,
    payload: &CreateAppliance,
) -> MainmanResult<Appliance> {
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
) -> MainmanResult<Appliance> {
    use crate::schema::appliance::dsl::*;

    let conn = pool.get()?;
    let res = diesel::update(appliance.find(payload.hash))
        .set(payload)
        .get_result::<Appliance>(&conn)?;

    Ok(res)
}
