use actix_http::Payload;
use actix_web::{web::Data, FromRequest, HttpRequest};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use futures::future::{err, ok, Ready};
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable, Pool},
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
    views::entity_overview,
    MainmanResult,
};

mod handler;
pub mod routes;

#[derive(
    Debug, Associations, Serialize, Deserialize, Queryable, Identifiable,
)]
#[belongs_to(Organisation, foreign_key = "organisation")]
#[table_name = "entity"]
#[primary_key(uuid)]
pub struct Entity {
    pub uuid: uuid::Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
    pub description: Option<String>,
    pub organisation: i64,
}

#[derive(Debug, Serialize, Associations, Queryable)]
#[table_name = "entity_overview"]
#[belongs_to(Entity, foreign_key = "uuid")]
pub struct EntityOverview {
    #[serde(skip)]
    uuid: uuid::Uuid,
    #[serde(skip)]
    organisaiton: i64,
    pub pending_requests: i64,
    pub unfinished_events: i64,
    pub finished_events: i64,
}

#[derive(Debug, Serialize)]
pub struct EntityWithOverview {
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(flatten)]
    pub overview: EntityOverview,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "entity"]
pub struct NewEntity {
    name: String,
    description: String,
    #[serde(skip_deserializing)]
    organisation: i64,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "entity"]
pub struct PatchEntity {
    name: Option<String>,
    description: Option<String>,
}

impl Entity {
    pub fn get(
        uuid: Uuid,
        organisation: i64,
        conn: &Connection,
    ) -> MainmanResult<Entity> {
        Ok(entity::table
            .find(uuid)
            .filter(entity::organisation.eq(organisation))
            .first::<Entity>(conn)?)
    }

    pub fn get_with_overview(
        uuid: Uuid,
        organisation: i64,
        conn: &Connection,
    ) -> MainmanResult<EntityWithOverview> {
        Ok(entity::table
            .find(uuid)
            .filter(entity::organisation.eq(organisation))
            .inner_join(entity_overview::table)
            .first::<(Entity, EntityOverview)>(conn)
            .map(|(entity, overview)| EntityWithOverview {
                entity,
                overview,
            })?)
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
        payload: &Vec<i64>,
        conn: &Connection,
    ) -> MainmanResult<()> {
        diesel::delete(
            maintainer_entity::table.filter(
                maintainer_entity::entity
                    .eq(self.uuid)
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

impl FromRequest for Entity {
    type Error = crate::error::ErrorResponse;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let conn = &req.app_data::<Data<Pool>>().unwrap().get().unwrap();

        match req.match_info().load::<(i64, uuid::Uuid)>() {
            Ok((organisation_id, entity)) => {
                match Entity::get(entity, organisation_id, conn) {
                    Ok(entity) => ok(entity),
                    Err(e) => err(e.into()),
                }
            }
            Err(e) => err(e.into()),
        }
    }
}
