#![allow(incomplete_features)]
#![feature(specialization)]

#[macro_use]
extern crate value_box;

#[macro_use]
extern crate phlow;
extern crate phlow_extensions;

use phlow_extensions::CoreExtensions;
use std::ops::Deref;

import_extensions!(CoreExtensions);

#[test]
pub fn phlow_box_i32() {
    let value: i32 = 42;
    let mut value_box = phlow_box!(value);
    let phlow_object = value_box.phlow_object().unwrap();
    assert_eq!(phlow_object.value_type_name(), "i32");
    assert_eq!(phlow_object.value_ref::<i32>().unwrap().deref(), &value);

    assert_eq!(value_box.has_value(), true);

    let taken_value = value_box.take_value().unwrap();
    assert_eq!(taken_value, value);
    assert_eq!(value_box.has_value(), false);
    assert_eq!(phlow_object.has_value(), false);
}
