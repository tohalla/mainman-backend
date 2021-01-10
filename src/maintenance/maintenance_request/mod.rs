use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::maintenance_request;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "maintenance_request"]
pub struct MaintenanceRequest {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<i32>,
    pub entity: Uuid,
    pub description: Option<String>,
}
