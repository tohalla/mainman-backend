use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    schema::maintenance_request,
    MainmanResult,
};

#[derive(
    Debug, Serialize, Deserialize, Queryable, Identifiable, Associations,
)]
#[belongs_to(Entity, foreign_key = "entity")]
#[table_name = "maintenance_request"]
pub struct MaintenanceRequest {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<i32>,
    pub entity: Uuid,
    pub description: Option<String>,
    pub maintenance_trigger: Option<Uuid>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintenance_request"]
pub struct NewMaintenanceRequest {
    entity: Uuid,
    maintenance_trigger: Option<Uuid>,
    description: String,
}

impl MaintenanceRequest {
    pub fn get(
        id: i64,
        conn: &Connection,
    ) -> MainmanResult<MaintenanceRequest> {
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
