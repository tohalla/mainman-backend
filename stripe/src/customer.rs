use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    pub invoice_prefix: String,
}

#[derive(Debug, Serialize)]
pub struct NewCustomer<'a> {
    pub email: &'a str,
}

#[derive(Debug, Serialize)]
pub struct InvoiceSettings<'a> {
    pub default_payment_method: &'a str,
}

#[derive(Debug, Serialize)]
pub struct PatchCustomer<'a> {
    pub invoice_settings: InvoiceSettings<'a>,
}

impl Customer {
    pub async fn get(
        client: &Client,
        id: &str,
    ) -> Result<Self, crate::error::Error> {
        Ok(client.get(format!("/customers/{}", id)).await?)
    }

    pub async fn patch<'a>(
        &self,
        client: &Client,
        patch_customer: &PatchCustomer<'a>,
    ) -> Result<Customer, crate::error::Error> {
        Ok(client
            .post(format!("/customers/{}", self.id), patch_customer)
            .await?)
    }
}

impl<'a> NewCustomer<'a> {
    pub async fn create(
        &self,
        client: &Client,
    ) -> Result<Customer, crate::error::Error> {
        Ok(client.post("/customers".to_owned(), self).await?)
    }
}
