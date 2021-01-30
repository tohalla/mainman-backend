use diesel::prelude::*;
use futures::future::try_join_all;
use stripe::{price::Price, product::Product, Client};

use crate::{schema::plan, MainmanResult};

pub async fn initialize() -> () {
    let client = Client::new();

    info!("initialization started");
    if let Err(err) = initialize_plans(&client).await {
        error!("Error: {}. Failed to initialize plans", err);
    }
    info!("initialization finished");
}

async fn initialize_plans(client: &Client) -> MainmanResult<()> {
    let conn = &super::db::get_pool().get()?;

    info!(target: "initialization", "synchronizing stripe products with plans");
    let products = Product::list(client).await?;
    let product_update_fut =
        products.data.into_iter().map(|product| async move {
            diesel::update(plan::table)
                .set(plan::stripe_product.eq(product.id))
                .filter(plan::name.eq(product.name))
                .execute(conn)
        });
    try_join_all(product_update_fut).await?;

    info!(target: "initialization", "synchronizing stripe prices with plans");
    let prices = Price::list(client).await?;
    let prices_update_fut = prices.data.into_iter().map(|price| async move {
        diesel::update(plan::table)
            .set(plan::stripe_price.eq(price.id))
            .filter(plan::stripe_product.eq(price.product))
            .execute(conn)
    });
    try_join_all(prices_update_fut).await?;

    Ok(())
}
