use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    maintenance::maintenance_request::MaintenanceRequest,
    schema::{entity, maintenance_event, maintenance_task},
    MainmanResult,
};

use super::maintenance_task::NewMaintenanceTask;

mod handler;
pub mod routes;

#[derive(
    Debug, Serialize, Deserialize, Queryable, Identifiable, Associations,
)]
#[belongs_to(MaintenanceRequest, foreign_key = "maintenance_request")]
#[belongs_to(Entity, foreign_key = "entity")]
#[table_name = "maintenance_event"]
pub struct MaintenanceEvent {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub resolved_at: Option<NaiveDateTime>,
    pub description: Option<String>,
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
    }
}
