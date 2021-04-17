use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{db::Connection, schema::maintenance_task, MainmanResult};

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

    pub fn complete(&self, conn: &Connection) -> MainmanResult<Self> {
        Ok(diesel::update(self)
            .set(maintenance_task::resolved_at.eq(Some(Utc::now().naive_utc())))
            .filter(maintenance_task::accepted_at.is_not_null())
            .get_result::<Self>(conn)?)
    }
}
