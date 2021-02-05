use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    schema::maintenance_trigger,
    MainmanResult,
};

// TODO: add different types
// #[derive(Debug)]
// pub enum Type {
//     Manual,
// }

#[derive(
    Debug, Serialize, Deserialize, Queryable, Identifiable, Associations,
)]
#[belongs_to(Entity, foreign_key = "entity")]
#[table_name = "maintenance_trigger"]
#[primary_key(hash)]
pub struct MaintenanceTrigger {
    pub hash: Uuid,
    pub created_at: NaiveDateTime,
    pub entity: Uuid,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintenance_trigger"]
pub struct NewMaintenanceTrigger {
    pub entity: Uuid,
}

impl Creatable<MaintenanceTrigger> for NewMaintenanceTrigger {
    fn create(&self, conn: &Connection) -> MainmanResult<MaintenanceTrigger> {
        Ok(diesel::insert_into(maintenance_trigger::table)
            .values(self)
            .get_result::<MaintenanceTrigger>(conn)?)
    }
}
