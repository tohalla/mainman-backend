use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    organisation::Organisation,
    schema::{entity, maintenance_trigger, organisation, template},
    template::Template,
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
    pub template: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct DetailedMaintenanceTrigger {
    #[serde(flatten)]
    pub maintenance_trigger: MaintenanceTrigger,
    pub entity: Entity,
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

    pub fn organisation(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Organisation> {
        Ok(entity::table
            .find(self.entity)
            .inner_join(organisation::table)
            .select(organisation::all_columns)
            .first::<Organisation>(conn)?)
    }
}

impl DetailedMaintenanceTrigger {
    pub fn get(
        uuid: Uuid,
        conn: &Connection,
    ) -> MainmanResult<DetailedMaintenanceTrigger> {
        let (maintenance_trigger, entity) = maintenance_trigger::table
            .find(uuid)
            .inner_join(entity::table)
            .first::<(MaintenanceTrigger, Entity)>(conn)?;
        Ok(DetailedMaintenanceTrigger {
            entity,
            maintenance_trigger,
        })
    }

    pub fn template(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Option<Template>> {
        if let Some(template_id) = self.maintenance_trigger.template {
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

impl Creatable<MaintenanceTrigger> for NewMaintenanceTrigger {
    fn create(&self, conn: &Connection) -> MainmanResult<MaintenanceTrigger> {
        Ok(diesel::insert_into(maintenance_trigger::table)
            .values(self)
            .get_result::<MaintenanceTrigger>(conn)?)
    }
}
