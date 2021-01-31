use actix_web::web::{Data, Json};
use stripe::{
    card::{Card, NewCard},
    customer::{Customer, InvoiceSettings, PatchCustomer},
    Client,
};

use crate::{account::Account, auth::Claim, db::Pool, MainmanResponse};

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
