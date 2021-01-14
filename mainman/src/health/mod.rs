use serde::{Deserialize, Serialize};

pub mod handler;

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

