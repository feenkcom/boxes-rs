#[macro_use]
extern crate log;

pub use error::*;

pub use self::value_box::*;

mod error;
mod value_box;