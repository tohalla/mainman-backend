use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::Connection,
    entity::Entity,
    organisation::Organisation,
    schema::{entity, maintenance_event, maintenance_task, organisation, template},
    template::Template,
    MainmanResult,
};

use super::maintenance_event::MaintenanceEvent;

mod handler;
pub mod routes;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "maintenance_task"]
#[primary_key(uuid)]
pub struct MaintenanceTask {
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<NaiveDateTime>,
    pub maintenance_event: i64,
    pub maintainer: i64,
    pub is_available: bool,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintenance_task"]
pub struct NewMaintenanceTask {
    pub maintenance_event: i64,
    pub maintainer: i64,
    pub is_available: bool,
}

#[derive(Debug, Serialize)]
pub struct DetailedMaintenanceTask {
    #[serde(flatten)]
    pub maintenance_task: MaintenanceTask,
    pub entity: Entity,
    pub maintenance_event: MaintenanceEvent,
}

impl MaintenanceTask {
    pub fn get(uuid: uuid::Uuid, conn: &Connection) -> MainmanResult<Self> {
        Ok(maintenance_task::table.find(uuid).first::<Self>(conn)?)
    }

    pub fn accept(&self, conn: &Connection) -> MainmanResult<Self> {
        let mut task = diesel::update(self)
            .set(maintenance_task::accepted_at.eq(Some(Utc::now().naive_utc())))
            .filter(maintenance_task::is_available.eq(true))
            .get_result::<Self>(conn)?;

        diesel::update(
            maintenance_task::table
                .filter(maintenance_task::maintenance_event.eq(self.maintenance_event)),
        )
        .set(maintenance_task::is_available.eq(false))
        .execute(conn)?;

        task.is_available = false;

        Ok(task)
    }

    fn organisation(&self, conn: &Connection) -> MainmanResult<Organisation> {
        Ok(maintenance_event::table
            .find(self.maintenance_event)
            .inner_join(entity::table)
            .inner_join(organisation::table.on(organisation::id.eq(entity::organisation)))
            .select(organisation::all_columns)
            .first::<Organisation>(conn)?)
    }

    pub fn resolve(&self, conn: &Connection) -> MainmanResult<(Self, MaintenanceEvent)> {
        conn.build_transaction().read_write().run(|| {
            let ts = Some(Utc::now().naive_utc());

            Ok((
                diesel::update(self)
                    .set(maintenance_task::resolved_at.eq(ts))
                    // TODO: proper error on task resolved / not accepted
                    .filter(
                        maintenance_task::accepted_at
                            .is_not_null()
                            .and(maintenance_task::resolved_at.is_null()),
                    )
                    .get_result::<Self>(conn)?,
                diesel::update(maintenance_event::table.find(self.maintenance_event))
                    .set(maintenance_event::resolved_at.eq(ts))
                    .get_result::<MaintenanceEvent>(conn)?,
            ))
        })
    }
}

impl DetailedMaintenanceTask {
    pub fn get(uuid: Uuid, conn: &Connection) -> MainmanResult<DetailedMaintenanceTask> {
        let (maintenance_task, maintenance_event, entity) = maintenance_task::table
            .find(uuid)
            .inner_join(maintenance_event::table)
            .inner_join(entity::table.on(maintenance_event::entity.eq(entity::uuid)))
            .first::<(MaintenanceTask, MaintenanceEvent, Entity)>(conn)?;
        Ok(DetailedMaintenanceTask {
            maintenance_task,
            entity,
            maintenance_event,
        })
    }

    pub fn template(&self, conn: &Connection) -> MainmanResult<Option<Template>> {
        if let Some(template_id) = self.entity.maitenance_report_template {
            return Ok(Some(
                template::table
                    .find(template_id)
                    .filter(template::organisation.eq(self.entity.organisation))
                    .first::<Template>(conn)?,
            ));
        }
        Ok(None)
    }
}
