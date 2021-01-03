use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json;
use uuid::Uuid;

use crate::entity::Entity;
use crate::{
    db::Pool,
    schema::{maintainer, maintainer_entity},
    MainmanResult,
};

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
#[table_name = "maintainer_entity"]
#[belongs_to(Maintainer, foreign_key = "maintainer")]
#[belongs_to(Entity, foreign_key = "entity")]
pub struct MaintainerEntity {
    pub maintainer: i32,
    pub entity: Uuid,
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

pub fn find(pool: &Pool, id: i32) -> MainmanResult<Maintainer> {
    use crate::schema::maintainer::dsl;
    Ok(dsl::maintainer.find(id).first::<Maintainer>(&pool.get()?)?)
}

pub fn get_all(
    pool: &Pool,
    organisation: i32,
) -> MainmanResult<Vec<Maintainer>> {
    use crate::schema::maintainer::dsl;
    Ok(dsl::maintainer
        .filter(dsl::organisation.eq(organisation))
        .load::<Maintainer>(&pool.get()?)?)
}

pub fn create(
    pool: &Pool,
    payload: &CreateMaintainer,
) -> MainmanResult<Maintainer> {
    use crate::schema::maintainer::dsl::*;
    Ok(diesel::insert_into(maintainer)
        .values(payload)
        .get_result::<Maintainer>(&pool.get()?)?)
}

pub fn patch(
    pool: &Pool,
    payload: &PatchMaintainer,
) -> MainmanResult<Maintainer> {
    use crate::schema::maintainer::dsl::*;
    Ok(diesel::update(maintainer.find(payload.id))
        .set(payload)
        .get_result::<Maintainer>(&pool.get()?)?)
}
