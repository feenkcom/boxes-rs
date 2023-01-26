#![cfg_attr(feature = "phlow", allow(incomplete_features))]
#![cfg_attr(feature = "phlow", feature(specialization))]

#[macro_use]
extern crate log;

#[cfg(feature = "phlow")]
extern crate phlow;

pub use error::*;

pub use self::value_box::*;
use self::value_box_container::*;
#[cfg(feature = "phlow")]
use self::value_box_phlow::*;

mod error;
mod value_box;
mod value_box_container;
#[cfg(feature = "phlow")]
mod value_box_phlow;

#[macro_export]
#[cfg(not(feature = "phlow"))]
macro_rules! value_box {
    ($var:expr) => {{
        value_box::ValueBox::new($var)
    }};
}

#[macro_export]
#[cfg(feature = "phlow")]
macro_rules! value_box {
    ($var:expr) => {{
        {
            let value = $var;
            let phlow_type_fn = crate::phlow_type_fn_of_val(&value);
            value_box::ValueBox::new_phlow(value, phlow_type_fn)
        }
    }};
}