use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    maintainer::{Maintainer, MaintainerEntity},
    maintenance::{
        maintenance_request::MaintenanceRequest,
        maintenance_trigger::MaintenanceTrigger,
    },
    organisation::Organisation,
    schema::{
        entity, maintainer, maintainer_entity, maintenance_request,
        maintenance_trigger,
    },
    MainmanResult,
};

pub mod handler;
pub mod routes;

#[derive(
    Debug, Associations, Serialize, Deserialize, Queryable, Identifiable,
)]
#[belongs_to(Organisation, foreign_key = "organisation")]
#[table_name = "entity"]
#[primary_key(hash)]
pub struct Entity {
    pub hash: uuid::Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
    pub description: Option<String>,
    pub organisation: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "entity"]
pub struct NewEntity {
    name: String,
    description: String,
    #[serde(skip_deserializing)]
    organisation: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "entity"]
pub struct PatchEntity {
    name: Option<String>,
    description: Option<String>,
}

impl Entity {
    pub fn get(
        hash: Uuid,
        organisation: i32,
        conn: &Connection,
    ) -> MainmanResult<Entity> {
        Ok(entity::table
            .find(hash)
            .filter(entity::organisation.eq(organisation))
            .first::<Entity>(conn)?)
    }

    pub fn maintainers(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Vec<Maintainer>> {
        Ok(MaintainerEntity::belonging_to(self)
            .inner_join(maintainer::table)
            .select(maintainer::all_columns)
            .load::<Maintainer>(conn)?)
    }

    pub fn patch(
        &self,
        payload: &PatchEntity,
        conn: &Connection,
    ) -> MainmanResult<Entity> {
        Ok(diesel::update(self)
            .set(payload)
            .get_result::<Entity>(conn)?)
    }

    pub fn maintenance_requests(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Vec<MaintenanceRequest>> {
        Ok(MaintenanceRequest::belonging_to(self)
            .select(maintenance_request::all_columns)
            .load::<MaintenanceRequest>(conn)?)
    }

    pub fn maintenance_triggers(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Vec<MaintenanceTrigger>> {
        Ok(MaintenanceTrigger::belonging_to(self)
            .select(maintenance_trigger::all_columns)
            .load::<MaintenanceTrigger>(conn)?)
    }

    pub fn delete_maintainers(
        &self,
        payload: &Vec<i32>,
        conn: &Connection,
    ) -> MainmanResult<()> {
        diesel::delete(
            maintainer_entity::table.filter(
                maintainer_entity::entity
                    .eq(self.hash)
                    .and(maintainer_entity::maintainer.eq_any(payload)),
            ),
        )
        .execute(conn)?;
        Ok(())
    }
}

impl Creatable<Entity> for NewEntity {
    fn create(&self, conn: &Connection) -> MainmanResult<Entity> {
        Ok(diesel::insert_into(entity::table)
            .values(self)
            .get_result::<Entity>(conn)?)
    }
}
