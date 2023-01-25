#![allow(non_snake_case)]

#[cfg(feature = "array-box")]
extern crate array_box;
#[cfg(feature = "geometry-box")]
extern crate geometry_box;
#[cfg(feature = "phlow")]
#[macro_use]
extern crate phlow;
#[cfg(feature = "string-box")]
extern crate string_box;

#[cfg(feature = "phlow")]
use phlow_extensions::CoreExtensions;

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

#[no_mangle]
pub extern "C" fn boxer_test() -> bool {
    return true;
}

#[cfg(feature = "phlow")]
import_extensions!(CoreExtensions);

#[no_mangle]
#[cfg(feature = "phlow")]
pub fn boxer_value_box_to_phlow_object(
    value_box: *mut value_box::ValueBox<&'static (dyn std::any::Any + 'static)>,
) -> *mut std::ffi::c_void {
    println!("value_box: {:?}", value_box);
    let mut value_box = std::mem::ManuallyDrop::new(unsafe { Box::from_raw(value_box) });
    println!("has_value: {}", value_box.has_value());

    value_box
        .phlow_object()
        .map(|phlow_object| {
            value_box::ValueBox::new(phlow_object).into_raw() as *mut std::ffi::c_void
        })
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
#[cfg(not(feature = "phlow"))]
pub fn boxer_value_box_to_phlow_object(
    _value_box: *mut value_box::ValueBox<std::ffi::c_void>,
) -> *mut std::ffi::c_void {
    std::ptr::null_mut()
}
