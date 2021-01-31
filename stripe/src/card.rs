use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub brand: String,
    pub country: String,
    pub customer: Option<String>,
    pub cvc_check: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub fingerprint: String,
    pub funding: String,
    pub last4: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewCard {
    pub source: String,
}

impl Card {
    pub async fn list(
        client: &Client,
        customer: &str,
    ) -> Result<crate::List<Self>, crate::error::Error> {
        Ok(client
            .get(format!("/customers/{}/sources", customer))
            .await?)
    }
}

impl NewCard {
    pub async fn create(
        &self,
        client: &Client,
        customer: &str,
    ) -> Result<Card, crate::error::Error> {
        Ok(client
            .post(format!("/customers/{}/sources", customer), self)
            .await?)
    }
}
