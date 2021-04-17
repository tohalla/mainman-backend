use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct SetupIntent {
    pub id: String,
    pub payment_method: Option<String>,
    pub customer: Option<String>,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewSetupIntent<'a> {
    pub payment_method: &'a str,
    pub customer: &'a str,
}

impl<'a> NewSetupIntent<'a> {
    pub async fn create(&self, client: &Client) -> Result<SetupIntent, crate::error::Error> {
        Ok(client.post("/setup_intents".to_owned(), self).await?)
    }
}
