use crate::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMethod {
    pub id: String,
    pub customer: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FilterPaymentMethods<'a> {
    pub customer: &'a str,
    #[serde(rename = "type")]
    pub payment_method_type: &'a str,
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
}
