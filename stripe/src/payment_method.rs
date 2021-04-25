use crate::{
    card::Card,
    customer::{Customer, InvoiceSettings, PatchCustomer},
    Client,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMethod {
    pub id: String,
    pub customer: Option<String>,

    pub card: Option<Card>,
}

#[derive(Debug, Serialize)]
pub struct FilterPaymentMethods<'a> {
    pub customer: &'a str,
    #[serde(rename = "type")]
    pub payment_method_type: &'a str,
}

#[derive(Debug, Serialize)]
struct AttachPaymentMethod<'a> {
    pub customer: &'a str,
}

impl PaymentMethod {
    pub async fn list<'a>(
        client: &Client,
        filter: &FilterPaymentMethods<'a>,
    ) -> Result<crate::List<Self>, crate::error::Error> {
        Ok(client
            .get_query("/payment_methods".to_owned(), filter)
            .await?)
    }

    /// Attaches payment method to a customer
    ///
    /// # Arguments
    ///
    /// * `client` = Stripe client
    /// * `customer_id` = the identifier of the customer the payment method will be attached to
    pub async fn attach(
        &self,
        client: &Client,
        customer: &Customer,
    ) -> Result<(), crate::error::Error> {
        client
            .post::<PaymentMethod, _>(
                format!("payment_methods/{}/attach", self.id),
                &AttachPaymentMethod {
                    customer: &customer.id,
                },
            )
            .await?;

        // make as default payment method if the customer did not have default one set previously
        if customer.invoice_settings.default_payment_method.is_none() {
            customer
                .patch(
                    client,
                    &PatchCustomer {
                        invoice_settings: InvoiceSettings {
                            default_payment_method: Some(self.id.to_owned()),
                            ..Default::default()
                        },
                    },
                )
                .await?;
        }

        Ok(())
    }

    pub async fn detach(client: &Client, id: &str) -> Result<Self, crate::error::Error> {
        Ok(client
            .send(format!("/payment_methods/{}/detach", id))
            .await?)
    }
}
