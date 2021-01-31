use chrono::NaiveDateTime;
use diesel::prelude::*;
use stripe::{
    customer::{Customer, NewCustomer},
    Client,
};

use crate::{
    db::{Connection, Creatable},
    schema::{account, card},
    MainmanResult,
};

pub mod handler;
pub mod routes;

#[derive(
    Debug, Serialize, Deserialize, Queryable, Associations, Identifiable,
)]
#[table_name = "account"]
pub struct Account {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    #[serde(skip)]
    pub password: Vec<u8>,
    #[serde(skip)]
    stripe_customer: Option<String>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "account"]
pub struct NewAccount<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
    password: &'a [u8],
}

#[derive(Debug, Serialize, Queryable, Associations, Identifiable)]
#[table_name = "card"]
struct Card {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub account: i32,
}

impl Account {
    pub fn get(id: i32, conn: &Connection) -> MainmanResult<Self> {
        Ok(account::dsl::account.find(id).first(conn)?)
    }

    fn set_stripe_customer(
        &self,
        conn: &Connection,
        stripe_customer: String,
    ) -> MainmanResult<Self> {
        Ok(diesel::update(self)
            .set(account::stripe_customer.eq(stripe_customer))
            .get_result(conn)?)
    }

    pub async fn stripe_customer(
        &self,
        conn: &Connection,
        stripe_client: &Client,
    ) -> MainmanResult<Customer> {
        if let Some(customer) = &self.stripe_customer {
            return Ok(Customer::get(stripe_client, customer).await?.into());
        }
        let customer = NewCustomer { email: &self.email }
            .create(&Client::new())
            .await?;
        self.set_stripe_customer(conn, customer.id.to_owned())?;

        Ok(customer)
    }

    pub fn add_card(
        &self,
        conn: &Connection,
        card_id: &str,
    ) -> MainmanResult<()> {
        diesel::insert_into(card::table)
            .values((card::account.eq(self.id), card::id.eq(card_id)))
            .execute(conn)?;
        Ok(())
    }
}

impl Creatable<Account> for NewAccount<'_> {
    fn create(&self, conn: &Connection) -> MainmanResult<Account> {
        Ok(diesel::insert_into(account::dsl::account)
            .values(self)
            .get_result(conn)?)
    }
}
