use crate::client::Client;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub invoice_prefix: String,
    pub invoice_settings: InvoiceSettings,
}

#[derive(Clone, Debug, Serialize)]
pub struct CustomerDetails {
    pub invoice_settings: InvoiceSettings,
}

#[derive(Clone, Debug, Serialize)]
pub struct NewCustomer<'a> {
    pub email: &'a str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<InvoiceSettingsCustomField>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvoiceSettingsCustomField {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatchCustomer {
    pub invoice_settings: InvoiceSettings,
}

impl Customer {
    pub async fn get(client: &Client, id: &str) -> Result<Self, crate::error::Error> {
        Ok(client.get(format!("/customers/{}", id)).await?)
    }

    pub async fn patch(
        &self,
        client: &Client,
        patch_customer: &PatchCustomer,
    ) -> Result<Customer, crate::error::Error> {
        Ok(client
            .post(format!("/customers/{}", self.id), patch_customer)
            .await?)
    }
}

impl<'a> NewCustomer<'a> {
    pub async fn create(&self, client: &Client) -> Result<Customer, crate::error::Error> {
        Ok(client.post("/customers".to_owned(), self).await?)
    }
}

impl Into<CustomerDetails> for Customer {
    fn into(self) -> CustomerDetails {
        CustomerDetails {
            invoice_settings: self.invoice_settings,
        }
    }
}

impl Default for InvoiceSettings {
    fn default() -> Self {
        Self {
            default_payment_method: None,
            footer: None,
            custom_fields: None,
        }
    }
}
