use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    organisation::Organisation,
    schema::{entity, maintainer, maintainer_entity},
    MainmanResult,
};

mod handler;
pub mod routes;

#[derive(
    Debug, Serialize, Deserialize, Queryable, Associations, Identifiable,
)]
#[table_name = "maintainer"]
#[belongs_to(Organisation, foreign_key = "organisation")]
pub struct Maintainer {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub organisation: i64,
    pub account: Option<i64>,
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
    pub maintainer: i64,
    #[serde(skip)]
    pub organisation: i64,
}

joinable!(maintainer_entity -> maintainer(maintainer));

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintainer"]
pub struct NewMaintainer {
    account: Option<i64>,
    #[serde(skip_deserializing)]
    organisation: i64,
    details: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "maintainer"]
pub struct PatchMaintainer {
    account: Option<i64>,
    details: Option<serde_json::Value>,
}

impl Maintainer {
    pub fn get(
        id: i64,
        organisation: i64,
        conn: &Connection,
    ) -> MainmanResult<Maintainer> {
        Ok(maintainer::table
            .find(id)
            .filter(maintainer::organisation.eq(organisation))
            .first::<Maintainer>(conn)?)
    }

    pub fn entities(&self, conn: &Connection) -> MainmanResult<Vec<Entity>> {
        Ok(MaintainerEntity::belonging_to(self)
            .inner_join(entity::table)
            .select(entity::all_columns)
            .load::<Entity>(conn)?)
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

    pub fn delete_entities(
        &self,
        payload: &Vec<Uuid>,
        conn: &Connection,
    ) -> MainmanResult<()> {
        diesel::delete(
            maintainer_entity::table.filter(
                maintainer_entity::maintainer
                    .eq(self.id)
                    .and(maintainer_entity::entity.eq_any(payload)),
            ),
        )
        .execute(conn)?;
        Ok(())
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
