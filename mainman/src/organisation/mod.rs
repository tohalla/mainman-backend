use account_role::AccountRole;
use chrono::NaiveDateTime;
use diesel::{prelude::*, sql_query, sql_types::Integer};
use invite::OrganisationInvite;

use crate::{
    account::Account,
    account::PublicAccount,
    db::{Connection, Creatable},
    entity::Entity,
    maintainer::Maintainer,
    schema::{
        self, account, entity, maintainer, organisation, organisation_account,
        organisation_invite, template,
    },
    template::Template,
    MainmanResult,
};

pub mod account_role;
mod handler;
pub mod invite;
pub mod plan;
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
    pub plan: i32,
}

#[derive(
    Debug, Serialize, Identifiable, Queryable, Associations, Insertable,
)]
#[table_name = "organisation_account"]
#[belongs_to(Account, foreign_key = "account")]
#[belongs_to(Organisation, foreign_key = "organisation")]
#[primary_key(account, organisation)]
pub struct OrganisationAccount {
    pub organisation: i32,
    pub account: i32,
    pub account_role: Option<i32>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "organisation"]
pub struct NewOrganisation {
    name: String,
    organisation_identifier: String,
    locale: String,
    plan: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "organisation"]
pub struct PatchOrganisation {
    name: Option<String>,
    organisation_identifier: Option<String>,
    locale: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct OrganisationOverview {
    #[sql_type = "Integer"]
    maintainers: i32,
    #[sql_type = "Integer"]
    entities: i32,
    #[sql_type = "Integer"]
    accounts: i32,
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

    pub fn overview(
        &self,
        conn: &Connection,
    ) -> MainmanResult<OrganisationOverview> {
        Ok(sql_query(
            "SELECT maintainers::INTEGER, entities::INTEGER, accounts::INTEGER FROM organisation_overview WHERE id = $1::INTEGER",
        )
        .bind::<Integer, _>(self.id)
        .get_result(conn)?)
    }

    pub fn all(
        account_id: i32,
        conn: &Connection,
    ) -> MainmanResult<Vec<Organisation>> {
        Ok(organisation_account::table
            .inner_join(organisation::table)
            .select(organisation::all_columns)
            .filter(organisation_account::account.eq(account_id))
            .load::<Organisation>(conn)?)
    }

    pub fn maintainers(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Vec<Maintainer>> {
        Ok(Maintainer::belonging_to(self)
            .select(maintainer::all_columns)
            .load::<Maintainer>(conn)?)
    }

    pub fn entities(&self, conn: &Connection) -> MainmanResult<Vec<Entity>> {
        Ok(Entity::belonging_to(self)
            .select(entity::all_columns)
            .load::<Entity>(conn)?)
    }

    pub fn templates(&self, conn: &Connection) -> MainmanResult<Vec<Template>> {
        Ok(Template::belonging_to(self)
            .select(template::all_columns)
            .load::<Template>(conn)?)
    }

    pub fn invites(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Vec<OrganisationInvite>> {
        Ok(OrganisationInvite::belonging_to(self)
            .select(organisation_invite::all_columns)
            .load::<OrganisationInvite>(conn)?)
    }

    pub fn accounts(
        &self,
        conn: &Connection,
    ) -> MainmanResult<Vec<PublicAccount>> {
        Ok(OrganisationAccount::belonging_to(self)
            .inner_join(account::table)
            .left_join(schema::account_role::table)
            .select((
                (
                    account::id,
                    account::first_name,
                    account::last_name,
                    account::email,
                ),
                schema::account_role::all_columns.nullable(),
            ))
            .load::<(
                (i32, Option<String>, Option<String>, String),
                Option<AccountRole>,
            )>(conn)?
            .into_iter()
            .map(|((id, first_name, last_name, email), role)| PublicAccount {
                role,
                id,
                first_name,
                last_name,
                email,
            })
            .collect())
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

impl Creatable<OrganisationAccount> for OrganisationAccount {
    fn create(&self, conn: &Connection) -> MainmanResult<OrganisationAccount> {
        Ok(diesel::insert_into(organisation_account::table)
            .values(self)
            .get_result::<OrganisationAccount>(conn)?)
    }
}

impl Creatable<Organisation> for NewOrganisation {
    fn create(&self, conn: &Connection) -> MainmanResult<Organisation> {
        Ok(diesel::insert_into(organisation::table)
            .values(self)
            .get_result::<Organisation>(conn)?)
    }
}
