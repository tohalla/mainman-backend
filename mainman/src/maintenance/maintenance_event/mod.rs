use chrono::NaiveDateTime;

use crate::schema::maintenance_event;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "maintenance_event"]
pub struct MaintenanceEvent {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub resolved_at: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub maintenance_request: i64,
}
