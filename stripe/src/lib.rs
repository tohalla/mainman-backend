#[macro_use]
extern crate serde;
extern crate serde_json;

mod client;
pub mod customer;
pub mod error;

pub use client::Client;
