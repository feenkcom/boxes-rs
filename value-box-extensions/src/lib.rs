#![feature(specialization)]

#[macro_use]
extern crate phlow;

use phlow::PhlowView;
use std::any::{type_name, TypeId};

define_extensions!(ValueBoxExtensions);
import_extensions!(ValueBoxExtensions);

#[phlow::extensions(ValueBoxExtensions, "ValueBox<T>")]
impl<T: 'static> ValueBox<T> {
    #[phlow::view]
    pub fn type_for(_this: &ValueBox<T>, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Type")
            .items(|_value: &ValueBox<T>, _| {
                phlow_all!(vec![
                    format!("Type: {}", type_name::<T>()),
                    format!("Type id: {:?}", TypeId::of::<T>()),
                ])
            })
    }
}
