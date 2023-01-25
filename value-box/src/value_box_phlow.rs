use crate::value_box_container::ValueBoxContainer;
use phlow::{AnyValue, PhlowObject};
use std::any::Any;

pub(crate) struct PhlowValue {
    pub(crate) container: PhlowValueContainer,
}

pub(crate) enum PhlowValueContainer {
    Lazy(LazyPhlowObject),
    Object(PhlowObject),
}

impl PhlowValue {
    pub fn new<T: Any>(value: T, phlow_type_fn: fn() -> phlow::PhlowType) -> Self {
        Self {
            container: PhlowValueContainer::Lazy(LazyPhlowObject {
                value: AnyValue::object(value),
                phlow_type_fn,
            }),
        }
    }
}

impl PhlowValue {
    pub(crate) fn phlow_object(&mut self) -> Option<PhlowObject> {
        let mut should_become_object = false;
        let phlow_object = match &mut self.container {
            PhlowValueContainer::Lazy(value) => {
                let phlow_type = (value.phlow_type_fn)();
                let any_value = std::mem::replace(&mut value.value, AnyValue::None);
                should_become_object = true;
                Some(PhlowObject::new(any_value, phlow_type, vec![], None))
            }
            PhlowValueContainer::Object(value) => Some(value.clone()),
        };
        if should_become_object {
            self.container = PhlowValueContainer::Object(phlow_object.clone().unwrap());
        }
        phlow_object
    }
}

impl<T: Any> ValueBoxContainer<T> for PhlowValue {
    fn replace_value(&mut self, object: T) -> Option<T> {
        match &mut self.container {
            PhlowValueContainer::Lazy(value) => value.replace_value(object),
            PhlowValueContainer::Object(value) => {
                <PhlowObject as ValueBoxContainer<T>>::replace_value(value, object)
            }
        }
    }

    fn take_value(&mut self) -> Option<T> {
        match &mut self.container {
            PhlowValueContainer::Lazy(value) => value.take_value(),
            PhlowValueContainer::Object(value) => {
                <PhlowObject as ValueBoxContainer<T>>::take_value(value)
            }
        }
    }

    fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        match &self.container {
            PhlowValueContainer::Lazy(value) => value.clone_value(),
            PhlowValueContainer::Object(value) => {
                <PhlowObject as ValueBoxContainer<T>>::clone_value(value)
            }
        }
    }

    fn as_ptr(&self) -> *const T {
        match &self.container {
            PhlowValueContainer::Lazy(value) => value.as_ptr(),
            PhlowValueContainer::Object(value) => {
                <PhlowObject as ValueBoxContainer<T>>::as_ptr(value)
            }
        }
    }

    fn has_value(&self) -> bool {
        match &self.container {
            PhlowValueContainer::Lazy(value) => {
                <LazyPhlowObject as ValueBoxContainer<T>>::has_value(value)
            }
            PhlowValueContainer::Object(value) => {
                <PhlowObject as ValueBoxContainer<T>>::has_value(value)
            }
        }
    }
}

pub(crate) struct LazyPhlowObject {
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

    fn as_ptr(&self) -> *const T {
        self.value.as_ptr() as *const T
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

    fn as_ptr(&self) -> *const T {
        PhlowObject::value_ptr(self) as *const T
    }

    fn has_value(&self) -> bool {
        PhlowObject::has_value(self)
    }
}