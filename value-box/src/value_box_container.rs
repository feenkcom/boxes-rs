use std::any::Any;

pub(crate) trait ValueBoxContainer<T: Any> {
    fn replace_value(&mut self, object: T) -> Option<T>;
    fn take_value(&mut self) -> Option<T>;
    fn clone_value(&self) -> Option<T>
    where
        T: Clone;
    fn has_value(&self) -> bool;
}

impl<T: Any> ValueBoxContainer<T> for Option<Box<T>> {
    fn replace_value(&mut self, object: T) -> Option<T> {
        self.replace(Box::new(object)).map(|boxed| *boxed)
    }

    fn take_value(&mut self) -> Option<T> {
        self.take().map(|boxed| *boxed)
    }

    fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        Clone::clone(self).map(|boxed| *boxed)
    }

    fn has_value(&self) -> bool {
        self.is_some()
    }
}
