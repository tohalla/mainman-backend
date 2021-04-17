use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    error::ErrorResponse,
    maintenance::maintenance_request::MaintenanceRequest,
    schema::{entity, maintenance_event, maintenance_request, maintenance_task},
    MainmanResult,
};

use super::maintenance_task::NewMaintenanceTask;

mod handler;
pub mod routes;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(MaintenanceRequest, foreign_key = "maintenance_request")]
#[belongs_to(Entity, foreign_key = "entity")]
#[table_name = "maintenance_event"]
pub struct MaintenanceEvent {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub resolved_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_request: Option<i64>,
    pub entity: uuid::Uuid,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintenance_event"]
pub struct NewMaintenanceEvent {
    pub maintenance_request: Option<i64>,
    pub description: Option<String>,
    #[serde(skip)]
    pub entity: uuid::Uuid,
}

impl Creatable<MaintenanceEvent> for NewMaintenanceEvent {
    fn create(&self, conn: &Connection) -> MainmanResult<MaintenanceEvent> {
        conn.build_transaction()
            .read_write()
            .run::<_, ErrorResponse, _>(|| {
                if let Some(request_id) = self.maintenance_request {
                    let request = maintenance_request::table
                        .find(request_id)
                        // TODO: return proper error instead of 404 when request already processed
                        .filter(maintenance_request::processed_at.is_null())
                        .first::<MaintenanceRequest>(conn)?;
                    diesel::update(&request)
                        .set(maintenance_request::processed_at.eq(Some(Utc::now().naive_utc())))
                        .execute(conn)?;
                }

                let event = diesel::insert_into(maintenance_event::table)
                    .values(self)
                    .get_result::<MaintenanceEvent>(conn)?;

                let entity = entity::table.find(event.entity).first::<Entity>(conn)?;
                // create maintenance_task for each maintainer
                diesel::insert_into(maintenance_task::table)
                    .values(
                        entity
                            .maintainers(&conn)?
                            .into_iter()
                            .map(|maintainer| NewMaintenanceTask {
                                maintenance_event: event.id,
                                maintainer: maintainer.id,
                                is_available: true,
                            })
                            .collect::<Vec<_>>(),
                    )
                    .execute(conn)?;
                // TODO: email maintainers

                Ok(event)
            })
    }
}
