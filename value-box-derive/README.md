# value-box-derive

Procedural macros for the `value-box` crate family.

Currently this crate provides the `#[value_box_derive::ffi]` attribute for rewriting simple borrowed Rust functions into FFI-facing wrappers that use `BorrowedPtr<T>`.
