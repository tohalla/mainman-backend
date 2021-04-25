#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_qs;
#[macro_use]
extern crate log;

mod client;

pub mod card;
pub mod customer;
pub mod error;
pub mod payment_method;
pub mod price;
pub mod product;
pub mod setup_intent;

pub use client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
}
