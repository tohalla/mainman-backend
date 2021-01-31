use actix_web::web::{Data, Json};
use futures::future::join;
use stripe::{
    customer::Customer,
    payment_method::{FilterPaymentMethods, PaymentMethod},
    setup_intent::NewSetupIntent,
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

#[post("stripe/payment_methods")]
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
            payment_method: &payment_method.id.to_owned(),
        }
        .create(client),
        (*payment_method).attach(client, &customer.id),
    )
    .await;
    attach_fut?;
    setup_intent_fut?;

    Ok(payment_method.into_inner().into())
}
