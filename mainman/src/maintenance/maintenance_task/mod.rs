use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::maintenance_task;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "maintenance_task"]
#[primary_key(uuid)]
pub struct MaintenanceTask {
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub accepted_at: Option<NaiveDateTime>,
    pub resolved_at: Option<NaiveDateTime>,
    pub maintenance_event: i64,
    pub maintainer: i64,
    pub is_available: bool,
}
