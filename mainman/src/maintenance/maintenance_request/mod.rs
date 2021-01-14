use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    error::Error,
    schema::maintenance_request,
    MainmanResult,
};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "maintenance_request"]
pub struct MaintenanceRequest {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub updated_by: Option<i32>,
    pub entity: Uuid,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintenance_request"]
pub struct NewMaintenanceRequest {
    entity: Uuid,
    description: String,
}

impl MaintenanceRequest {
    pub fn get(
        id: i64,
        conn: &Connection,
    ) -> MainmanResult<MaintenanceRequest> {
        Ok(maintenance_request::table.find(id).first::<Self>(conn)?)
    }

    pub fn by_entity(
        entity: Uuid,
        conn: &Connection,
    ) -> MainmanResult<Vec<Self>> {
        use crate::schema::maintenance_request::dsl;
        Ok(dsl::maintenance_request
            .filter(dsl::entity.eq(entity))
            .load::<MaintenanceRequest>(conn)
            .map_err(|_| Error::NotFoundError)?)
    }
}

impl Creatable<MaintenanceRequest> for NewMaintenanceRequest {
    fn create(&self, conn: &Connection) -> MainmanResult<MaintenanceRequest> {
        Ok(diesel::insert_into(maintenance_request::table)
            .values(self)
            .get_result::<MaintenanceRequest>(conn)?)
    }
}
