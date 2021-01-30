use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};
pub struct Client {
    client: actix_web::client::Client,
}

static STRIPE_URL: &str = "https://api.stripe.com/v1";

impl Client {
    pub fn new() -> Self {
        Client {
            client: actix_web::client::ClientBuilder::new()
                .timeout(Duration::from_secs(10))
                .basic_auth(Self::secret(), None)
                .finish(),
        }
    }

    fn secret() -> String {
        std::env::var("STRIPE_SECRET").unwrap().to_string()
    }

    fn uri(path: String) -> String {
        if path.starts_with("/") {
            return format!("{}{}", STRIPE_URL, path);
        }
        format!("{}/{}", STRIPE_URL, path)
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        path: String,
    ) -> Result<T, crate::error::Error> {
        Ok(serde_json::from_slice::<T>(
            &*self
                .client
                .get(Self::uri(path))
                .send()
                .await?
                .body()
                .await?,
        )?)
    }

    pub async fn post<T: DeserializeOwned, U: Serialize>(
        &self,
        path: String,
        payload: &U,
    ) -> Result<T, crate::error::Error> {
        Ok(serde_json::from_slice::<T>(
            &*self
                .client
                .post(Self::uri(path))
                .send_form(payload)
                .await?
                .body()
                .await?,
        )?)
    }
}
