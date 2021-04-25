use actix_web::web::{Data, Json, Path};
use futures::future::join;
use stripe::{
    customer::{CustomerDetails, PatchCustomer},
    payment_method::{FilterPaymentMethods, PaymentMethod},
    setup_intent::NewSetupIntent,
    Client,
};

use crate::{account::Account, auth::Claim, db::Pool, MainmanResponse};

#[get("stripe")]
pub async fn customer_details(pool: Data<Pool>, claim: Claim) -> MainmanResponse<CustomerDetails> {
    let conn = &pool.get()?;

    let customer_details: CustomerDetails = Account::get(claim.account_id, conn)?
        .stripe_customer(conn, &Client::new())
        .await?
        .into();

    Ok(customer_details.into())
}

#[patch("stripe")]
pub async fn patch_customer(
    pool: Data<Pool>,
    claim: Claim,
    payload: Json<PatchCustomer>,
) -> MainmanResponse<CustomerDetails> {
    let client = &Client::new();
    let conn = &pool.get()?;

    let details: CustomerDetails = Account::get(claim.account_id, conn)?
        .stripe_customer(conn, &Client::new())
        .await?
        .patch(client, &payload)
        .await?
        .into();

    Ok(details.into())
}

#[get("stripe/payment-methods")]
pub async fn get_payment_methods(
    pool: Data<Pool>,
    claim: Claim,
) -> MainmanResponse<Vec<PaymentMethod>> {
    let conn = &pool.get()?;
    let client = &Client::new();
    let account = Account::get(claim.account_id, conn)?;

    let stripe_customer = match account.stripe_customer {
        Some(stripe_customer) => stripe_customer,
        None => account.stripe_customer(conn, client).await?.id,
    };

    Ok(PaymentMethod::list(
        client,
        &FilterPaymentMethods {
            payment_method_type: "card",
            customer: &stripe_customer,
        },
    )
    .await?
    .data
    .into())
}

#[post("stripe/payment-methods")]
pub async fn create_payment_method(
    pool: Data<Pool>,
    claim: Claim,
    payment_method: Json<PaymentMethod>,
) -> MainmanResponse<PaymentMethod> {
    let conn = &pool.get()?;
    let client = &Client::new();
    let account = Account::get(claim.account_id, conn)?;
    let customer = account.stripe_customer(conn, client).await?;

    let (setup_intent_fut, attach_fut) = join(
        NewSetupIntent {
            customer: &customer.id,
            payment_method: &payment_method.id,
        }
        .create(client),
        (*payment_method).attach(client, &customer),
    )
    .await;
    attach_fut?;
    setup_intent_fut?;

    Ok(payment_method.into_inner().into())
}

#[delete("stripe/payment-methods/{id}")]
pub async fn detach_payment_method(id: Path<String>) -> MainmanResponse<PaymentMethod> {
    let client = &Client::new();
    Ok(PaymentMethod::detach(client, &id).await?.into())
}
