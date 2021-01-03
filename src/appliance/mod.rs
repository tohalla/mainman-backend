use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{db::Connection, error::Error, schema::appliance, MainmanResult};

pub mod handler;
pub mod routes;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "appliance"]
#[primary_key(hash)]
pub struct Appliance {
    pub hash: uuid::Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
    pub description: Option<String>,
    pub organisation: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "appliance"]
pub struct NewAppliance {
    name: String,
    description: String,
    #[serde(skip_deserializing)]
    organisation: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "appliance"]
pub struct PatchAppliance {
    name: Option<String>,
    description: Option<String>,
}

impl Appliance {
    pub fn get(hash: Uuid, conn: &Connection) -> MainmanResult<Appliance> {
        Ok(appliance::table.find(hash).first::<Appliance>(conn)?)
    }

    pub fn by_organisation(
        organisation: i32,
        conn: &Connection,
    ) -> MainmanResult<Vec<Appliance>> {
        use crate::schema::appliance::dsl;

        Ok(dsl::appliance
            .filter(dsl::organisation.eq(organisation))
            .load::<Appliance>(conn)
            .map_err(|_| Error::NotFoundError)?)
    }

    pub fn patch(
        &self,
        payload: &PatchAppliance,
        conn: &Connection,
    ) -> MainmanResult<Appliance> {
        Ok(diesel::update(self)
            .set(payload)
            .get_result::<Appliance>(conn)?)
    }
}

impl NewAppliance {
    pub fn insert(&self, conn: &Connection) -> MainmanResult<Appliance> {
        Ok(diesel::insert_into(appliance::table)
            .values(self)
            .get_result::<Appliance>(conn)?)
    }
}
