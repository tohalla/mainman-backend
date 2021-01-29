use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    pub invoice_prefix: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewCustomer<'a> {
    pub email: &'a str,
}

impl Customer {
    pub async fn get(
        client: &Client,
        id: String,
    ) -> Result<Self, crate::error::Error> {
        Ok(client.get(format!("/customers/{}", id)).await?)
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
