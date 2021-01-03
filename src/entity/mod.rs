use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{db::Connection, error::Error, schema::entity, MainmanResult};

pub mod handler;
pub mod routes;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "entity"]
#[primary_key(hash)]
pub struct Entity {
    pub hash: uuid::Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
    pub description: Option<String>,
    pub organisation: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "entity"]
pub struct NewEntity {
    name: String,
    description: String,
    #[serde(skip_deserializing)]
    organisation: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "entity"]
pub struct PatchEntity {
    name: Option<String>,
    description: Option<String>,
}

impl Entity {
    pub fn get(hash: Uuid, conn: &Connection) -> MainmanResult<Entity> {
        Ok(entity::table.find(hash).first::<Entity>(conn)?)
    }

    pub fn by_organisation(
        organisation: i32,
        conn: &Connection,
    ) -> MainmanResult<Vec<Entity>> {
        use crate::schema::entity::dsl;

        Ok(dsl::entity
            .filter(dsl::organisation.eq(organisation))
            .load::<Entity>(conn)
            .map_err(|_| Error::NotFoundError)?)
    }

    pub fn patch(
        &self,
        payload: &PatchEntity,
        conn: &Connection,
    ) -> MainmanResult<Entity> {
        Ok(diesel::update(self)
            .set(payload)
            .get_result::<Entity>(conn)?)
    }
}

impl NewEntity {
    pub fn insert(&self, conn: &Connection) -> MainmanResult<Entity> {
        Ok(diesel::insert_into(entity::table)
            .values(self)
            .get_result::<Entity>(conn)?)
    }
}
