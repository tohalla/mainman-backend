use actix_web::{web::Json, Responder};

#[get("/health")]
pub async fn get_health() -> impl Responder {
    Json(super::HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
