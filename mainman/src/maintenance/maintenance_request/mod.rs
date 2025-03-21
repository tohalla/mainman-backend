use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    schema::maintenance_request,
    MainmanResult,
};

mod handler;
pub mod routes;

#[derive(Deserialize)]
pub struct Filter {
    pub processed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(Entity, foreign_key = "entity")]
#[table_name = "maintenance_request"]
pub struct MaintenanceRequest {
    pub id: i64,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<i64>,
    pub entity: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_trigger: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintenance_request"]
pub struct NewMaintenanceRequest {
    #[serde(skip)]
    pub entity: Uuid,
    #[serde(skip)]
    pub maintenance_trigger: Uuid,
    pub description: String,
}

impl MaintenanceRequest {
    pub fn get(id: i64, conn: &Connection) -> MainmanResult<MaintenanceRequest> {
        Ok(maintenance_request::table.find(id).first::<Self>(conn)?)
    }
}

impl Creatable<MaintenanceRequest> for NewMaintenanceRequest {
    fn create(&self, conn: &Connection) -> MainmanResult<MaintenanceRequest> {
        Ok(diesel::insert_into(maintenance_request::table)
            .values(self)
            .get_result::<MaintenanceRequest>(conn)?)
    }
}
