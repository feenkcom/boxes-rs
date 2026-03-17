# ValueBox
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/value-box.svg
[crates-url]: https://crates.io/crates/value-box
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/feenkcom/boxes-rs/blob/main/LICENSE

`value-box` allows developers to pass Rust-allocated structures over ffi.
For migration from the old `ValueBox` API, see [`MIGRATION_V4.md`](../MIGRATION_V4.md).

The `value-box` crate handles most typical use-cases when creating ffi bindings to rust libraries such as:
 - Return newly allocated Rust structures passing ownership to the caller.
 - Receiving the previously created value box and calling associated functions taking the rust structure by reference, mutable reference, cloning the value or taking the value.
 - Finally, dropping the box with the Rust structure in it.
 - Supports `Box<dyn MyTrait>`.
 - `OwnedPtr<T>` and `BorrowedPtr<T>` are `#[repr(transparent)]` pointer wrappers
 - Error handling via custom `Error` and `Result`.

## Example:

```rust
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub fn library_object_create() -> OwnedPtr<MyObject> {
    OwnedPtr::new(MyObject::new())
}

#[unsafe(no_mangle)]
pub fn library_object_is_something(object: BorrowedPtr<MyObject>) -> bool {
    object
        // with_ref_ok() wraps the returned value into Result:Ok,
        // hence the name
        .with_ref_ok(|object| object.is_something())
        .unwrap_or(false)
}

#[unsafe(no_mangle)]
pub fn library_object_try_something(object: BorrowedPtr<MyObject>) {
    object
        // with_ref() expects the closure to return a Result
        .with_ref(|object| object.try_something())
        .log();
}

#[unsafe(no_mangle)]
pub fn library_object_by_ref(object: BorrowedPtr<MyObject>) {
    object.with_ref_ok(|object| object.by_ref()).log();
}

#[unsafe(no_mangle)]
pub fn library_object_by_mut(mut object: BorrowedPtr<MyObject>) {
    object.with_mut_ok(|object| object.by_mut()).log();
}

#[unsafe(no_mangle)]
pub fn library_object_by_value(object: OwnedPtr<MyObject>) {
    object.take_value().map(|object| object.by_value()).log();
}

#[unsafe(no_mangle)]
pub fn library_object_by_value_clone(object: BorrowedPtr<MyObject>) {
    object
        .with_clone_ok(|object| object.by_value())
        .log();
}

#[unsafe(no_mangle)]
pub fn library_object_release(object: OwnedPtr<MyObject>) {
    drop(object);
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
    pub fn try_something(&self) -> Result<()> {
        Ok(())
    }
}
```

## License

Copyright feenk gmbh.

Licensed under MIT. See [LICENSE](LICENSE).