use std::any::Any;

pub(crate) enum Container<T> {
    Value(Option<T>),
    #[cfg(feature = "phlow")]
    PhlowValue(crate::PhlowValue<T>),
}

pub(crate) trait ValueBoxContainer<T: Any> {
    fn replace_value(&mut self, object: T) -> Option<T>;
    fn take_value(&mut self) -> Option<T>;
    fn clone_value(&self) -> Option<T>
    where
        T: Clone;
    fn as_ptr(&self) -> *const T;
    fn has_value(&self) -> bool;
}

impl<T: Any> ValueBoxContainer<T> for Option<T> {
    fn replace_value(&mut self, object: T) -> Option<T> {
        self.replace(object)
    }

    fn take_value(&mut self) -> Option<T> {
        self.take()
    }

    fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        Clone::clone(self)
    }

    fn as_ptr(&self) -> *const T {
        self.as_ref()
            .map_or(std::ptr::null(), |reference| reference as *const T)
    }

    fn has_value(&self) -> bool {
        self.is_some()
    }
}

impl<T: Any> ValueBoxContainer<T> for Container<T> {
    fn replace_value(&mut self, object: T) -> Option<T> {
        match self {
            Container::Value(value) => value.replace_value(object),
            #[cfg(feature = "phlow")]
            Container::PhlowValue(value) => value.replace_value(object),
        }
    }

    fn take_value(&mut self) -> Option<T> {
        match self {
            Container::Value(value) => value.take_value(),
            #[cfg(feature = "phlow")]
            Container::PhlowValue(value) => value.take_value(),
        }
    }

    fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Container::Value(value) => value.clone_value(),
            #[cfg(feature = "phlow")]
            Container::PhlowValue(value) => value.clone_value(),
        }
    }

    fn as_ptr(&self) -> *const T {
        match self {
            Container::Value(value) => value.as_ptr(),
            #[cfg(feature = "phlow")]
            Container::PhlowValue(value) => value.as_ptr(),
        }
    }

    fn has_value(&self) -> bool {
        match self {
            Container::Value(value) => value.has_value(),
            #[cfg(feature = "phlow")]
            Container::PhlowValue(value) => value.has_value(),
        }
    }
}
