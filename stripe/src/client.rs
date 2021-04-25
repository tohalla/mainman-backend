use actix_web::client::Connector;
use serde::{de::DeserializeOwned, Serialize};
use std::{sync::Arc, time::Duration};

use crate::error::Error;

pub struct Client {
    client: actix_web::client::Client,
}

static STRIPE_URL: &str = "https://api.stripe.com/v1";

impl Client {
    pub fn new() -> Self {
        let mut rustls_config = rustls::ClientConfig::new();
        rustls_config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        Client {
            client: actix_web::client::ClientBuilder::new()
                .connector(
                    Connector::new()
                        .timeout(Duration::from_secs(10))
                        .rustls(Arc::new(rustls_config))
                        .finish(),
                )
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

    pub async fn get<T: DeserializeOwned>(&self, path: String) -> Result<T, Error> {
        let mut res = self.client.get(Self::uri(path)).send().await?;
        Ok(serde_json::from_slice::<T>(&res.body().await?)?)
    }

    pub async fn get_query<T: DeserializeOwned, U: Serialize>(
        &self,
        path: String,
        query: U,
    ) -> Result<T, Error> {
        let mut res = self
            .client
            .get(Self::uri(path))
            .query(&query)
            .map_err(|_| Error::GenericError)?
            .send()
            .await?;
        Ok(serde_json::from_slice::<T>(&res.body().await?)?)
    }

    pub async fn post<T: DeserializeOwned, U: Serialize + std::fmt::Debug>(
        &self,
        path: String,
        payload: &U,
    ) -> Result<T, crate::error::Error> {
        let url = Self::uri(path);
        let body = serde_qs::to_string(payload)?;
        debug!("POST {} <- {}", url, body);
        let mut res = self.client.post(url).send_body(body).await?;
        Ok(serde_json::from_slice::<T>(&res.body().await?)?)
    }

    pub async fn send<T: DeserializeOwned>(&self, path: String) -> Result<T, crate::error::Error> {
        let mut res = self.client.post(Self::uri(path)).send().await?;
        Ok(serde_json::from_slice::<T>(&res.body().await?)?)
    }
}
