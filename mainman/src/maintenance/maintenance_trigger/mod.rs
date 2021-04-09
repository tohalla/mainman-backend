use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    db::{Connection, Creatable},
    entity::Entity,
    schema::{entity, maintenance_trigger, template, template_type},
    template::{template_type::MAINTENANCE_REQUEST, Template},
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

    pub fn template(&self, conn: &Connection) -> MainmanResult<Template> {
        Ok(match self.maintenance_trigger.template {
            Some(template_id) => {
                template::table
                    .filter(template::id.eq(template_id).and(
                        template::organisation.eq(self.entity.organisation),
                    ))
                    .first::<Template>(conn)
            }
            None => template::table
                .select(template::all_columns)
                .inner_join(
                    template_type::table.on(template_type::name
                        .eq(MAINTENANCE_REQUEST)
                        .and(template_type::id.eq(template::template_type))),
                )
                .filter(template::organisation.is_null())
                .first::<Template>(conn),
        }?)
    }
}

impl Creatable<MaintenanceTrigger> for NewMaintenanceTrigger {
    fn create(&self, conn: &Connection) -> MainmanResult<MaintenanceTrigger> {
        Ok(diesel::insert_into(maintenance_trigger::table)
            .values(self)
            .get_result::<MaintenanceTrigger>(conn)?)
    }
}
