use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recurring {
    pub interval: String,
    pub interval_count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Price {
    pub id: String,
    #[serde(skip_serializing)]
    pub product: String,
    pub currency: String,
    pub unit_amount: i32,
    #[serde(skip_serializing)]
    pub active: bool,
    pub recurring: Recurring,
}

impl Price {
    pub async fn list(client: &Client) -> Result<crate::List<Self>, crate::error::Error> {
        Ok(client.get("/prices".to_owned()).await?)
    }

    pub async fn get(client: &Client, id: String) -> Result<Self, crate::error::Error> {
        Ok(client.get(format!("/prices/{}", id)).await?)
    }
}
