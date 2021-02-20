use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    schema::maintenance_trigger,
    MainmanResult,
};

mod handler;
pub mod routes;

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
#[primary_key(uuid)]
pub struct MaintenanceTrigger {
    pub uuid: Uuid,
    pub created_at: NaiveDateTime,
    pub entity: Uuid,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "maintenance_trigger"]
pub struct NewMaintenanceTrigger {
    #[serde(skip)]
    pub entity: Uuid,
}

impl MaintenanceTrigger {
    pub fn get(uuid: Uuid, conn: &Connection) -> MainmanResult<Self> {
        Ok(maintenance_trigger::table
            .find(uuid)
            .first::<MaintenanceTrigger>(conn)?)
    }

    pub fn delete(
        entity: Uuid,
        trigger: Uuid,
        conn: &Connection,
    ) -> MainmanResult<()> {
        diesel::delete(
            maintenance_trigger::table.filter(
                maintenance_trigger::entity
                    .eq(entity)
                    .and(maintenance_trigger::uuid.eq(trigger)),
            ),
        )
        .execute(conn)?;
        Ok(())
    }
}

impl Creatable<MaintenanceTrigger> for NewMaintenanceTrigger {
    fn create(&self, conn: &Connection) -> MainmanResult<MaintenanceTrigger> {
        Ok(diesel::insert_into(maintenance_trigger::table)
            .values(self)
            .get_result::<MaintenanceTrigger>(conn)?)
    }
}
