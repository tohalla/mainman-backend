use actix_web::web::{Data, Json, Path};
use bcrypt::{hash, DEFAULT_COST};
use heck::TitleCase;
use stripe::{
    card::{Card, NewCard},
    customer::{Customer, InvoiceSettings, PatchCustomer},
    Client,
};

use super::*;
use crate::{auth::Claim, db::Pool, MainmanResponse};

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
    claim: Claim,
) -> MainmanResponse<Customer> {
    let conn = &pool.get()?;
    let account = Account::get(claim.account_id, conn)?;

    Ok(account.stripe_customer(conn, &Client::new()).await?.into())
}

#[get("stripe/cards")]
pub async fn get_cards(
    pool: Data<Pool>,
    claim: Claim,
) -> MainmanResponse<Vec<Card>> {
    let conn = &pool.get()?;
    let client = &Client::new();
    let account = Account::get(claim.account_id, conn)?;

    let stripe_customer = match account.stripe_customer {
        Some(stripe_customer) => stripe_customer,
        None => account.stripe_customer(conn, client).await?.id,
    };

    Ok(Card::list(client, &stripe_customer).await?.data.into())
}

#[post("stripe/cards")]
pub async fn create_card(
    pool: Data<Pool>,
    claim: Claim,
    card: Json<NewCard>,
) -> MainmanResponse<Card> {
    let conn = &pool.get()?;
    let client = &Client::new();
    let account = Account::get(claim.account_id, conn)?;
    let customer = account.stripe_customer(conn, client).await?;

    let card = card.into_inner().create(client, &customer.id).await?;
    account.add_card(conn, &card.id)?;
    // TODO: setting as default payment method should be optional (if not first card)
    customer
        .patch(
            client,
            &PatchCustomer {
                invoice_settings: InvoiceSettings {
                    default_payment_method: &card.id,
                },
            },
        )
        .await?;

    Ok(card.into())
}
