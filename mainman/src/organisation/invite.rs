use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    auth::Claim,
    db::{Connection, Creatable},
    organisation::Organisation,
    schema::{account, organisation_account, organisation_invite},
    MainmanResult,
};

use super::OrganisationAccount;

#[derive(
    Debug, Associations, Serialize, Deserialize, Queryable, Identifiable,
)]
#[belongs_to(Organisation, foreign_key = "organisation")]
#[table_name = "organisation_invite"]
#[primary_key(uuid)]
pub struct OrganisationInvite {
    pub uuid: uuid::Uuid,
    pub organisation: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable, Validate)]
#[table_name = "organisation_invite"]
pub struct NewOrganisationInvite {
    #[validate(email(message = "invalid_email"))]
    pub email: String,
    #[serde(skip_deserializing)]
    pub organisation: i32,
}

impl OrganisationInvite {
    pub fn get(
        organisation_id: i32,
        uuid: uuid::Uuid,
        conn: &Connection,
    ) -> MainmanResult<Self> {
        Ok(organisation_invite::table
            .find(uuid)
            .filter(organisation_invite::organisation.eq(organisation_id))
            .first::<OrganisationInvite>(conn)?)
    }

    pub fn accept(
        &self,
        claim: &Claim,
        conn: &Connection,
    ) -> MainmanResult<OrganisationAccount> {
        let account_id = account::table
            .select(account::id)
            .find(claim.account_id)
            .filter(account::email.eq(&self.email))
            .first::<i32>(conn)?;
        let organisation_account = OrganisationAccount::create(
            &OrganisationAccount {
                organisation: self.organisation,
                account: account_id,
                account_role: None,
            },
            conn,
        )?;
        self.delete(conn)?;
        Ok(organisation_account)
    }

    pub fn delete(&self, conn: &Connection) -> MainmanResult<()> {
        diesel::delete(self).execute(conn)?;
        Ok(())
    }
}

impl Creatable<OrganisationInvite> for NewOrganisationInvite {
    fn create(&self, conn: &Connection) -> MainmanResult<OrganisationInvite> {
        // check that account exists and is not part of the organisation already
        let (_, organisation_account) = account::table
            .filter(account::email.eq(&self.email))
            .left_join(organisation_account::table)
            .select((account::id, organisation_account::account.nullable()))
            .first::<(i32, Option<i32>)>(conn)?;

        if organisation_account.is_some() {
            return Err(crate::error::Error::default()
                .detail("account_in_organisation")
                .source("email")
                .into());
        }

        Ok(diesel::insert_into(organisation_invite::table)
            .values(self)
            .get_result::<OrganisationInvite>(conn)?)
    }
}
