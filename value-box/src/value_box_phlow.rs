use crate::value_box_container::ValueBoxContainer;
use phlow::{AnyValue, PhlowObject};
use std::any::Any;

pub enum PhlowValue {
    Lazy(LazyPhlowObject),
    Object(PhlowObject),
}

impl PhlowValue {
    pub fn new<T: Any>(value: T, phlow_type_fn: fn() -> phlow::PhlowType) -> Self {
        Self::Lazy(LazyPhlowObject {
            value: AnyValue::object(value),
            phlow_type_fn,
        })
    }
}

impl PhlowValue {
    pub(crate) fn phlow_object(&mut self) -> Option<PhlowObject> {
        let mut should_become_object = false;
        let phlow_object = match self {
            PhlowValue::Lazy(value) => {
                let phlow_type = (value.phlow_type_fn)();
                let any_value = std::mem::replace(&mut value.value, AnyValue::None);
                should_become_object = true;
                Some(PhlowObject::new(any_value, phlow_type, vec![], None))
            }
            PhlowValue::Object(value) => Some(value.clone()),
        };
        if should_become_object {
            let _ = std::mem::replace(self, PhlowValue::Object(phlow_object.clone().unwrap()));
        }
        phlow_object
    }
}

impl<T: Any> ValueBoxContainer<T> for PhlowValue {
    fn replace_value(&mut self, object: T) -> Option<T> {
        match self {
            PhlowValue::Lazy(value) => value.replace_value(object),
            PhlowValue::Object(value) => {
                <PhlowObject as ValueBoxContainer<T>>::replace_value(value, object)
            }
        }
    }

    fn take_value(&mut self) -> Option<T> {
        match self {
            PhlowValue::Lazy(value) => value.take_value(),
            PhlowValue::Object(value) => <PhlowObject as ValueBoxContainer<T>>::take_value(value),
        }
    }

    fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            PhlowValue::Lazy(value) => value.clone_value(),
            PhlowValue::Object(value) => <PhlowObject as ValueBoxContainer<T>>::clone_value(value),
        }
    }

    fn has_value(&self) -> bool {
        match self {
            PhlowValue::Lazy(value) => <LazyPhlowObject as ValueBoxContainer<T>>::has_value(value),
            PhlowValue::Object(value) => <PhlowObject as ValueBoxContainer<T>>::has_value(value),
        }
    }
}

#[repr(C)]
pub struct LazyPhlowObject {
    pub(crate) value: AnyValue,
    // A function used to create a phlow type of the generic type T
    phlow_type_fn: fn() -> phlow::PhlowType,
}

impl<T: Any> ValueBoxContainer<T> for LazyPhlowObject {
    fn replace_value(&mut self, object: T) -> Option<T> {
        let previous = std::mem::replace(&mut self.value, AnyValue::object(object));
        previous.take_value()
    }

    fn take_value(&mut self) -> Option<T> {
        let previous = std::mem::replace(&mut self.value, AnyValue::None);
        previous.take_value()
    }

    fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        self.value.clone_value()
    }

    fn has_value(&self) -> bool {
        self.value.has_value()
    }
}

impl<T: Any> ValueBoxContainer<T> for PhlowObject {
    fn replace_value(&mut self, object: T) -> Option<T> {
        PhlowObject::replace_value(self, object)
    }

    fn take_value(&mut self) -> Option<T> {
        PhlowObject::take_value(self)
    }

    fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        PhlowObject::clone_value(self)
    }

    fn has_value(&self) -> bool {
        PhlowObject::has_value(self)
    }
}
