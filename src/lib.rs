#[macro_use]
extern crate serde_derive;

#[cfg(feature = "client")]
pub mod client;
pub mod dto;
