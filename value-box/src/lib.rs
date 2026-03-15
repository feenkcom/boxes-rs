#[macro_use]
extern crate log;

pub use error::*;

pub use self::value_box::*;

mod error;
mod value_box;

#[macro_export]
macro_rules! value_box {
    ($var:expr) => {{
        value_box::ValueBox::new($var)
    }};
}
