#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMethod {
    pub id: String,
    pub customer: Option<String>,
}
