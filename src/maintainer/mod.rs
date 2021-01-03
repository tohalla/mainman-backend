use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json;
use uuid::Uuid;

use crate::{
    db::Connection,
    entity::Entity,
    error::Error,
    schema::{maintainer, maintainer_entity},
    MainmanResult,
};

pub mod handler;
pub mod routes;

#[derive(
    Debug, Serialize, Deserialize, Queryable, Associations, Identifiable,
)]
#[table_name = "maintainer"]
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
pub struct NewMaintainer {
    account: Option<i32>,
    organisation: i32,
    details: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "maintainer"]
pub struct PatchMaintainer {
    id: i32,
    account: Option<i32>,
    details: Option<serde_json::Value>,
}

impl Maintainer {
    pub fn get(id: i32, conn: &Connection) -> MainmanResult<Maintainer> {
        Ok(maintainer::table.find(id).first::<Maintainer>(conn)?)
    }

    pub fn by_organisation(
        organisation: i32,
        conn: &Connection,
    ) -> MainmanResult<Vec<Maintainer>> {
        use crate::schema::maintainer::dsl;

        Ok(dsl::maintainer
            .filter(dsl::organisation.eq(organisation))
            .load::<Maintainer>(conn)
            .map_err(|_| Error::NotFoundError)?)
    }

    pub fn patch(
        &self,
        payload: &PatchMaintainer,
        conn: &Connection,
    ) -> MainmanResult<Maintainer> {
        Ok(diesel::update(self)
            .set(payload)
            .get_result::<Maintainer>(conn)?)
    }
}

impl NewMaintainer {
    pub fn insert(&self, conn: &Connection) -> MainmanResult<Maintainer> {
        Ok(diesel::insert_into(maintainer::table)
            .values(self)
            .get_result::<Maintainer>(conn)?)
    }
}
