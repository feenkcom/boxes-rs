#![allow(non_snake_case)]

#[cfg(feature = "array-box")]
extern crate array_box;
#[cfg(feature = "geometry-box")]
extern crate geometry_box;
#[cfg(feature = "string-box")]
extern crate string_box;

#[cfg(feature = "value-box")]
pub use crate::value_box_ffi::*;
#[cfg(feature = "array-box")]
pub use array_box_ffi::*;
#[cfg(feature = "geometry-box")]
pub use geometry_box_ffi::*;
#[cfg(feature = "string-box")]
pub use string_box_ffi::*;

#[cfg(feature = "array-box")]
mod array_box_ffi;
#[cfg(feature = "geometry-box")]
mod geometry_box_ffi;
#[cfg(feature = "string-box")]
mod string_box_ffi;
#[cfg(feature = "value-box")]
mod value_box_ffi;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_test() -> bool {
    true
}
