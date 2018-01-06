#![cfg_attr(feature = "cargo-clippy", allow(clippy_pedantic))]

extern crate bytes;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub mod cero;
pub mod gr_types;
pub mod cero_types;
pub use cero::{ deserialize };
