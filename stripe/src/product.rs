use crate::client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
}

impl Product {
    pub async fn list(
        client: &Client,
    ) -> Result<crate::List<Self>, crate::error::Error> {
        Ok(client.get("/products".to_owned()).await?)
    }

    pub async fn get(
        client: &Client,
        id: String,
    ) -> Result<Self, crate::error::Error> {
        Ok(client.get(format!("/products/{}", id)).await?)
    }
}
