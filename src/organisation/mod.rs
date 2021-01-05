use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    account::Account,
    db::{Connection, Creatable},
    schema::{organisation, organisation_account},
    MainmanResult,
};

pub mod handler;
pub mod routes;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Queryable,
    Associations,
    AsChangeset,
    Identifiable,
)]
#[table_name = "organisation"]
pub struct Organisation {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
    pub organisation_identifier: Option<String>,
    pub locale: String,
    pub admin_account: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "organisation_account"]
#[belongs_to(Account, foreign_key = "account")]
#[belongs_to(Organisation, foreign_key = "organisation")]
pub struct OrganisationAccount {
    pub id: i32,
    pub organisation: i32,
    pub account: i32,
    pub account_role: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "organisation"]
pub struct NewOrganisation {
    name: String,
    organisation_identifier: String,
    locale: String,
    #[serde(skip_deserializing)]
    admin_account: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "organisation"]
pub struct PatchOrganisation {
    name: Option<String>,
    organisation_identifier: Option<String>,
    locale: Option<String>,
}

impl Organisation {
    pub fn get(
        organisation_id: i32,
        conn: &Connection,
    ) -> MainmanResult<Organisation> {
        Ok(organisation::table
            .find(organisation_id)
            .first::<Organisation>(conn)?)
    }

    pub fn all(
        account_id: i32,
        conn: &Connection,
    ) -> MainmanResult<Vec<Organisation>> {
        use crate::schema::organisation::dsl::*;
        Ok(organisation
            .filter(admin_account.eq(account_id))
            .load::<Organisation>(conn)?)
    }

    pub fn patch(
        &self,
        payload: &PatchOrganisation,
        conn: &Connection,
    ) -> MainmanResult<Self> {
        Ok(diesel::update(self)
            .set(payload)
            .get_result::<Organisation>(conn)?)
    }
}

impl Creatable<Organisation> for NewOrganisation {
    fn create(&self, conn: &Connection) -> MainmanResult<Organisation> {
        Ok(diesel::insert_into(organisation::table)
            .values(self)
            .get_result::<Organisation>(conn)?)
    }
}
