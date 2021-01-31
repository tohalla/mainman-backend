#[macro_use]
extern crate serde;
extern crate serde_json;

mod client;

pub mod card;
pub mod customer;
pub mod error;
pub mod price;
pub mod product;

pub use client::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
}
