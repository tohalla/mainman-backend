use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    error::Error,
    schema::{entity, maintainer, maintainer_entity},
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

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Queryable,
    Associations,
    Insertable,
    Identifiable,
)]
#[table_name = "maintainer_entity"]
#[primary_key(maintainer, entity)]
#[belongs_to(Maintainer, foreign_key = "maintainer")]
#[belongs_to(Entity, foreign_key = "entity")]
pub struct MaintainerEntity {
    pub entity: Uuid,
    pub maintainer: i32,
    #[serde(skip)]
    pub organisation: i32,
}

joinable!(maintainer_entity -> maintainer(maintainer));
joinable!(maintainer_entity -> entity(entity));

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintainer"]
pub struct NewMaintainer {
    account: Option<i32>,
    #[serde(skip_deserializing)]
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

    pub fn by_entity(
        hash: &uuid::Uuid,
        conn: &Connection,
    ) -> MainmanResult<Vec<Maintainer>> {
        use crate::schema::maintainer_entity::dsl;
        Ok(maintainer_entity::table
            .inner_join(maintainer::table)
            .filter(dsl::entity.eq(hash))
            .select(maintainer::all_columns)
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

impl Creatable<Maintainer> for NewMaintainer {
    fn create(&self, conn: &Connection) -> MainmanResult<Maintainer> {
        Ok(diesel::insert_into(maintainer::table)
            .values(self)
            .get_result::<Maintainer>(conn)?)
    }
}

impl Creatable<MaintainerEntity> for MaintainerEntity {
    fn create(&self, conn: &Connection) -> MainmanResult<MaintainerEntity> {
        Ok(diesel::insert_into(maintainer_entity::table)
            .values(self)
            .get_result::<MaintainerEntity>(conn)?)
    }
}

impl Creatable<Vec<MaintainerEntity>> for [MaintainerEntity] {
    fn create(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Vec<MaintainerEntity>> {
        Ok(diesel::insert_into(maintainer_entity::table)
            .values(self)
            .load::<MaintainerEntity>(conn)?)
    }
}
