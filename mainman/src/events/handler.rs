use std::sync::Mutex;

use actix_http::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use actix_web::{web::Data, HttpResponse};

use crate::auth::Claim;

use super::Broadcaster;

#[get("")]
pub fn connect(broker: Data<Mutex<Broadcaster>>, claim: Claim) -> HttpResponse {
    match broker.lock() {
        Ok(mut broker) => HttpResponse::Ok()
            .set_header(CONTENT_TYPE, "text/event-stream")
            .set_header(CACHE_CONTROL, "no-cache")
            .streaming(broker.connect(claim.account_id)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
