use actix_web::web::{Data, Json, Path};
use bcrypt::{hash, DEFAULT_COST};
use heck::TitleCase;
use stripe::{
    customer::{Customer, NewCustomer},
    Client,
};

use super::*;
use crate::{db::Pool, MainmanResponse};

#[derive(Clone, Debug, Deserialize)]
pub struct NewAccountPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[post("")]
pub async fn create_account(
    pool: Data<Pool>,
    payload: Json<NewAccountPayload>,
) -> MainmanResponse<Account> {
    Ok(NewAccount {
        email: &payload.email,
        first_name: &payload.first_name.to_title_case(),
        last_name: &payload.last_name.to_title_case(),
        password: hash(&payload.password, DEFAULT_COST)?.as_bytes(),
    }
    .create(&pool.get()?)?
    .into())
}

#[get("")]
pub async fn get_account(
    pool: Data<Pool>,
    account_id: Path<i32>,
) -> MainmanResponse<Account> {
    Ok(Account::get(*account_id, &pool.get()?)?.into())
}

#[get("stripe")]
pub async fn get_customer_details(
    pool: Data<Pool>,
    account_id: Path<i32>,
) -> MainmanResponse<Customer> {
    let conn = &pool.get()?;
    let account = Account::get(*account_id, conn)?;

    Ok(account.stripe_customer(conn, &Client::new()).await?.into())
}

    account.set_stripe_customer(conn, customer.id.to_owned())?;

    Ok(customer.into())
}
