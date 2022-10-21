# ValueBox
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/value-box.svg
[crates-url]: https://crates.io/crates/value-box
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/feenkcom/boxes-rs/blob/main/LICENSE

`ValueBox` allows developers to pass Rust-allocated structures over ffi.
The `value-box` crate handles most typical use-cases when creating ffi bindings to rust libraries such as:
 - Return newly allocated Rust structures passing ownership to the caller.
 - Receiving the previously created value box and calling associated functions taking the rust structure by reference, mutable reference, cloning the value or taking the value.
 - Finally, dropping the box with the Rust structure in it.
 - Supports `Box<dyn MyTrait>`.
 - `ValueBox` is defined as `#[transparent]`
 - Error handling via custom `Error` and `Result`.

## Example:

```rust
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn library_object_create() -> *mut ValueBox<MyObject> {
    ValueBox::new(MyObject::new()).into_raw()
}

#[no_mangle]
pub fn library_object_is_something(object: *mut ValueBox<MyObject>) -> bool {
    object
        .to_ref()
        .map(|object| object.is_something())
        .unwrap_or(false)
}

#[no_mangle]
pub fn library_object_by_ref(object: *mut ValueBox<MyObject>) {
    object.to_ref().map(|object| object.by_ref()).log();
}

#[no_mangle]
pub fn library_object_by_mut(object: *mut ValueBox<MyObject>) {
    object.to_ref().map(|mut object| object.by_mut()).log();
}

#[no_mangle]
pub fn library_object_by_value(object: *mut ValueBox<MyObject>) {
    object.take_value().map(|object| object.by_value()).log();
}

#[no_mangle]
pub fn library_object_by_value_clone(object: *mut ValueBox<MyObject>) {
    object
        .to_ref()
        .map(|object| object.clone().by_value())
        .log();
}

#[no_mangle]
pub fn library_object_release(object: *mut ValueBox<MyObject>) {
    object.release();
}

#[derive(Debug, Clone)]
pub struct MyObject {}
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
```