use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[unsafe(no_mangle)]
pub fn library_object_create() -> *mut ValueBox<MyObject> {
    ValueBox::new(MyObject::new()).into_raw()
}

#[unsafe(no_mangle)]
pub fn library_object_is_something(object: *mut ValueBox<MyObject>) -> bool {
    object
        .with_ref_ok(|object| object.is_something())
        .unwrap_or(false)
}

#[unsafe(no_mangle)]
pub fn library_object_by_ref(object: *mut ValueBox<MyObject>) {
    object.with_ref_ok(|object| object.by_ref()).log();
}

#[unsafe(no_mangle)]
pub fn library_object_by_mut(object: *mut ValueBox<MyObject>) {
    object.with_mut_ok(|object| object.by_mut()).log();
}

#[unsafe(no_mangle)]
pub fn library_object_by_value(object: *mut ValueBox<MyObject>) {
    object.take_value().map(|object| object.by_value()).log();
}

#[unsafe(no_mangle)]
pub fn library_object_by_value_clone(object: *mut ValueBox<MyObject>) {
    object.with_clone_ok(|object| object.by_value()).log();
}

#[unsafe(no_mangle)]
pub fn library_object_release(object: *mut ValueBox<MyObject>) {
    object.release();
}

#[derive(Debug, Clone)]
pub struct MyObject {}

impl Default for MyObject {
    fn default() -> Self {
        Self::new()
    }
}

impl MyObject {
    pub fn new() -> Self {
        Self {}
    }

    pub fn by_ref(&self) {}
    pub fn by_mut(&mut self) {}
    pub fn by_value(self) {}
    pub fn is_something(&self) -> bool {
        true
    }
}
