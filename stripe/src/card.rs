#[derive(Debug, Deserialize, Serialize)]
pub struct Checks {
    pub address_line_check: Option<String>,
    pub address_postal_code_check: Option<String>,
    pub cvc_check: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThreeDSecureUsage {
    pub supported: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub brand: String,
    pub country: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub fingerprint: String,
    pub funding: String,
    pub last4: String,
    #[serde(skip_serializing)]
    pub checks: Checks,
    #[serde(skip_serializing)]
    pub three_d_secure_usage: ThreeDSecureUsage,
    #[serde(skip_serializing)]
    pub generated_from: Option<String>,
}
