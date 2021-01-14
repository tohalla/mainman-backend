use actix_web::{self, http::StatusCode, HttpRequest, HttpResponse, Responder};
use futures::future::{err, ok, Ready};
use serde::Serialize;

use crate::error;

#[derive(Debug, Serialize, Default)]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct Meta {}

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize> {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl<T: Serialize> Response<T> {
    pub fn new(data: T) -> Self {
        Response {
            status_code: StatusCode::OK,
            data,
            links: None,
            meta: None,
        }
    }

    pub fn set_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn set_links(mut self, links: Links) -> Self {
        self.links = Some(links);
        self
    }

    pub fn set_meta(mut self, meta: Meta) -> Self {
        self.meta = Some(meta);
        self
    }
}

impl<T: Serialize> From<T> for Response<T> {
    fn from(data: T) -> Self {
        Response::new(data)
    }
}

impl<T: Serialize> Responder for Response<T> {
    type Error = error::Error;
    type Future = Ready<actix_http::Result<actix_http::Response, Self::Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        match serde_json::to_string(&self) {
            Ok(body) => ok(HttpResponse::build(self.status_code)
                .content_type("application/json")
                .body(body)),
            Err(_) => err(error::Error::InternalServerError(None)),
        }
    }
}
