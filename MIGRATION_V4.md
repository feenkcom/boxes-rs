# Migration to v4

This guide covers migration from the old `ValueBox` API, including the deprecated helper methods, to the current `OwnedPtr<T>` and `BorrowedPtr<T>` API.

It is written both for humans and for Agent-driven migrations. The goal is to make ownership decisions mechanical wherever possible.

## High-level changes

Version 4 separates ownership from borrowing:

- `OwnedPtr<T>` means the callee owns the pointer.
- `BorrowedPtr<T>` means the callee may borrow the pointee for the duration of a closure.
- `ValueBox<T>` is no longer part of the public API.
- `with_value()` / `with_value_ok()` exist only on `OwnedPtr<T>`.
- It is intentionally not possible to derive `BorrowedPtr<T>` from `OwnedPtr<T>`.

This removes the old footgun where ownership-taking APIs could be reached through raw-pointer-style helper methods.

## Core rule

Choose the pointer type from the function contract, not from the old type.

- If the function only reads the pointee, use `BorrowedPtr<T>`.
- If the function mutates the pointee in place, use `BorrowedPtr<T>` and bind the parameter as `mut`.
- If the function clones the pointee and keeps the original alive, use `BorrowedPtr<T>`.
- If the function destroys the pointee, use `OwnedPtr<T>`.
- If the function takes the pointee out and consumes it, use `OwnedPtr<T>`.
- If the function creates a new pointee and returns it, return `OwnedPtr<T>`.

The old `*mut ValueBox<T>` type does not tell you which one is correct. The body does.

## Constructor and signature changes

Old:

```rust
use value_box::{ValueBox, ValueBoxPointer};

#[unsafe(no_mangle)]
pub fn library_object_create() -> *mut ValueBox<MyObject> {
    ValueBox::new(MyObject::new()).into_raw()
}

#[unsafe(no_mangle)]
pub fn library_object_is_something(object: *mut ValueBox<MyObject>) -> bool {
    object.with_ref_ok(|object| object.is_something()).unwrap_or(false)
}

#[unsafe(no_mangle)]
pub fn library_object_release(object: *mut ValueBox<MyObject>) {
    object.release();
}
```

New:

```rust
use value_box::{BorrowedPtr, OwnedPtr};

#[unsafe(no_mangle)]
pub fn library_object_create() -> OwnedPtr<MyObject> {
    OwnedPtr::new(MyObject::new())
}

#[unsafe(no_mangle)]
pub fn library_object_is_something(object: BorrowedPtr<MyObject>) -> bool {
    object.with_ref_ok(|object| object.is_something()).unwrap_or(false)
}

#[unsafe(no_mangle)]
pub fn library_object_release(object: OwnedPtr<MyObject>) {
    drop(object);
}
```

## Ownership rules

Old code often used one raw pointer type for everything:

- creation
- borrowing
- mutation
- cloning
- ownership transfer
- destruction

In v4, pick the pointer type based on the contract:

- return `OwnedPtr<T>` from constructors
- accept `BorrowedPtr<T>` for read-only or mutable borrowed access
- accept `OwnedPtr<T>` for functions that consume the object
- call `with_value()` / `with_value_ok()` only on `OwnedPtr<T>`

If you need `&T`, `&mut T`, or `T`, use:

- `BorrowedPtr::with_ref`
- `BorrowedPtr::with_mut`
- `BorrowedPtr::with_clone`
- `OwnedPtr::with_value`

## Decision table

Use this table when migrating signatures:

| Old pattern in function body             | New argument/return type | New operation                          |
|------------------------------------------|--------------------------|----------------------------------------|
| `ValueBox::new(value).into_raw()`        | `OwnedPtr<T>` return     | `OwnedPtr::new(value)`                 |
| `ptr.with_ref...`                        | `BorrowedPtr<T>` arg     | `with_ref` / `with_ref_ok`             |
| `ptr.with_mut...`                        | `BorrowedPtr<T>` arg     | `with_mut` / `with_mut_ok`             |
| `ptr.with_clone...`                      | `BorrowedPtr<T>` arg     | `with_clone` / `with_clone_ok`         |
| `ptr.take_value()`                       | `OwnedPtr<T>` arg        | `with_value()` / `with_value_ok()`     |
| `ptr.release()`                          | `OwnedPtr<T>` arg        | `drop(ptr)`                            |
| function returns null on failure         | `OwnedPtr<T>` return     | `OwnedPtr::null()`                     |
| function accepts nullable borrowed input | `BorrowedPtr<T>` arg     | `BorrowedPtr<T>` already supports null |
| function accepts nullable owned input    | `OwnedPtr<T>` arg        | `OwnedPtr<T>` already supports null    |

## Raw pointer signature rewrites

These are the most common mechanical rewrites.

### Constructor

Old:

```rust
pub extern "C" fn foo_create() -> *mut ValueBox<Foo>
```

New:

```rust
pub extern "C" fn foo_create() -> OwnedPtr<Foo>
```

### Borrowed getter

Old:

```rust
pub extern "C" fn foo_get_x(foo: *mut ValueBox<Foo>) -> i32
```

New:

```rust
pub extern "C" fn foo_get_x(foo: BorrowedPtr<Foo>) -> i32
```

### Borrowed setter

Old:

```rust
pub extern "C" fn foo_set_x(foo: *mut ValueBox<Foo>, x: i32)
```

New:

```rust
pub extern "C" fn foo_set_x(mut foo: BorrowedPtr<Foo>, x: i32)
```

### Consuming call

Old:

```rust
pub extern "C" fn foo_consume(foo: *mut ValueBox<Foo>)
```

New:

```rust
pub extern "C" fn foo_consume(foo: OwnedPtr<Foo>)
```

### Drop function

Old:

```rust
pub extern "C" fn foo_drop(foo: *mut ValueBox<Foo>) {
    foo.release();
}
```

New:

```rust
pub extern "C" fn foo_drop(foo: OwnedPtr<Foo>) {
    drop(foo);
}
```

## Body rewrites

### Read-only access

Old:

```rust
pub fn foo_is_ready(foo: *mut ValueBox<Foo>) -> bool {
    foo.with_ref_ok(|foo| foo.is_ready()).unwrap_or(false)
}
```

New:

```rust
pub fn foo_is_ready(foo: BorrowedPtr<Foo>) -> bool {
    foo.with_ref_ok(|foo| foo.is_ready()).unwrap_or(false)
}
```

### Mutable borrowed access

Old:

```rust
pub fn foo_prepare(foo: *mut ValueBox<Foo>) {
    foo.with_mut_ok(|foo| foo.prepare()).log();
}
```

New:

```rust
pub fn foo_prepare(mut foo: BorrowedPtr<Foo>) {
    foo.with_mut_ok(|foo| foo.prepare()).log();
}
```

### Consume the pointee

Old:

```rust
pub fn foo_run(foo: *mut ValueBox<Foo>) {
    foo.with_value_ok(|foo| foo.run()).log();
}
```

New:

```rust
pub fn foo_run(foo: OwnedPtr<Foo>) {
    foo.with_value_ok(|foo| foo.run()).log();
}
```

### Clone the pointee

Old:

```rust
pub fn foo_clone_run(foo: *mut ValueBox<Foo>) {
    foo.with_clone_ok(|foo| foo.run()).log();
}
```

New:

```rust
pub fn foo_clone_run(foo: BorrowedPtr<Foo>) {
    foo.with_clone_ok(|foo| foo.run()).log();
}
```

## Method mapping

### Old `ValueBox` / `ValueBoxPointer`

- `ValueBox::new(value).into_raw()` -> `OwnedPtr::new(value)`
- `ValueBox::new(value)` -> `OwnedPtr::new(value)` if the result was immediately turned into an exported handle
- `*mut ValueBox<T>` argument used only for borrowing -> `BorrowedPtr<T>`
- `*mut ValueBox<T>` argument used for destruction -> `OwnedPtr<T>`
- `*mut ValueBox<T>` argument used for ownership transfer -> `OwnedPtr<T>`
- `ptr.release()` -> `drop(ptr)` where `ptr: OwnedPtr<T>`
- `ptr.take_value()` -> `ptr.with_value(...)` or `ptr.with_value_ok(...)` where `ptr: OwnedPtr<T>`
- `ptr.erase()` -> `borrowed.erase()` where `borrowed: BorrowedPtr<T>` and the result is `ErasedBorrowedPtr`
- `ptr.has_value()` on a borrowed handle -> `!ptr.is_null()` where `ptr: BorrowedPtr<T>`
- `ptr.has_value()` on a consuming/dropping handle -> `!ptr.is_null()` where `ptr: OwnedPtr<T>`

### Deprecated borrowed helpers

- `is_valid()` -> `!is_null()`
- `with_not_null(block)` -> `with_mut_ok(block).log()` or `with_mut_ok(block)` on `mut BorrowedPtr<T>`
- `with_not_null_return(default, block)` -> `with_mut_ok(block).unwrap_or(default)` on `mut BorrowedPtr<T>`
- `with_value(default, block)` -> `with_clone_ok(block).unwrap_or_else(|_| default())`
- `with_not_null_value(block)` -> `with_clone_ok(block).map(|value| block(value)).log()` or `with_clone_ok(block)`
- `with_not_null_value_return(default, block)` -> `with_clone_ok(block).unwrap_or(default)`

## `*mut c_void` and erased pointer cases

Some code erases typed boxes to `*mut c_void` or an erased borrowed handle.

### Old typed erase

Old:

```rust
let erased: ErasedBorrowedPtr = ptr.erase();
```

New:

```rust
let erased: ErasedBorrowedPtr = ptr.erase();
```

This remains valid on `BorrowedPtr<T>`, not on `OwnedPtr<T>`. `ErasedBorrowedPtr` is a transparent pointer wrapper, similar to `BorrowedPtr<T>`.

### Old erased borrowed API

If the old code accepted `*mut ValueBox<c_void>` or another erased borrowed-pointer representation only to check for null or pass the pointer through a closure, migrate to one of:

- `BorrowedPtr<c_void>` when the API is conceptually a borrowed typed handle erased to `c_void`
- `ErasedBorrowedPtr` when the API is intentionally type-erased

Examples:

Old:

```rust
pub extern "C" fn boxer_value_box_is_valid(ptr: *mut ValueBox<c_void>) -> bool
```

New:

```rust
pub extern "C" fn boxer_value_box_is_valid(ptr: BorrowedPtr<c_void>) -> bool
```

Old:

```rust
fn visit(ptr: ErasedBorrowedPtr) -> Result<()> {
    ptr.with_ptr(|raw| ...)
}
```

New:

```rust
fn visit(ptr: ErasedBorrowedPtr) -> Result<()> {
    ptr.with_ptr(|raw| ...)
}
```

No change there beyond using the explicit erased borrowed wrapper instead of an erased raw pointee type.

## Concrete rewrites

Old:

```rust
pub fn library_object_by_mut(object: *mut ValueBox<MyObject>) {
    object.with_not_null(|object| object.by_mut());
}
```

New:

```rust
pub fn library_object_by_mut(mut object: BorrowedPtr<MyObject>) {
    object.with_mut_ok(|object| object.by_mut()).log();
}
```

Old:

```rust
pub fn library_object_clone_call(object: *mut ValueBox<MyObject>) {
    object.with_not_null_value(|object| object.by_value());
}
```

New:

```rust
pub fn library_object_clone_call(object: BorrowedPtr<MyObject>) {
    object.with_clone_ok(|object| object.by_value()).log();
}
```

Old:

```rust
pub fn library_object_consume(object: *mut ValueBox<MyObject>) {
    object.with_value_ok(|object| object.by_value()).log();
}
```

New:

```rust
pub fn library_object_consume(object: OwnedPtr<MyObject>) {
    object.with_value_ok(|object| object.by_value()).log();
}
```

## How to classify ambiguous functions

When the correct replacement is unclear, inspect the body and classify it by these rules:

1. If the function calls `with_value`, `release`, or otherwise destroys the box, the parameter becomes `OwnedPtr<T>`.
2. If the function only calls methods through `&T`, the parameter becomes `BorrowedPtr<T>`.
3. If the function calls methods through `&mut T`, the parameter becomes `mut BorrowedPtr<T>`.
4. If the function clones `T` and uses the clone, the parameter becomes `BorrowedPtr<T>`.
5. If the function conditionally consumes ownership on one path, the parameter is still `OwnedPtr<T>`.
6. If a helper function is called, classify based on what that helper does, not on the old signature.

If you cannot prove that the pointee remains owned by the caller, prefer `OwnedPtr<T>`.

## What no longer works by design

These patterns are intentionally gone in v4:

- borrowing from `OwnedPtr<T>`
- extracting a raw pointer from `OwnedPtr<T>`
- turning `OwnedPtr<T>` into `BorrowedPtr<T>`
- calling `with_value()` through a borrowed pointer type
- treating one function parameter as both borrowed and owned in the same API shape
- using `OwnedPtr<T>` as a general-purpose pointer wrapper

`OwnedPtr<T>` is an ownership boundary.
`BorrowedPtr<T>` is a borrowing boundary.
Do not blur them back together during migration.

## Null handling

Null handling stays explicit:

- `BorrowedPtr<T>::null()` represents an absent borrowed pointer
- `OwnedPtr<T>::null()` represents an absent owned pointer
- `is_null()` matches the usual raw-pointer naming
- closure-based borrowed access still returns `NullPointer(...)` on null
- `with_value()` on `OwnedPtr::null()` still returns `NullPointer(...)`

If old code used `std::ptr::null_mut()` as a failure sentinel, migrate that to `OwnedPtr::null()` or `BorrowedPtr::null()` depending on the contract.

If a function only needs to inspect or mutate an object, make it take `BorrowedPtr<T>`.
If it needs to consume the object, make it take `OwnedPtr<T>`.

## Agents migration rules

When using Agents to migrate other repos, the default strategy should be:

1. Rewrite public signatures first.
2. Replace constructor returns with `OwnedPtr::new(...)`.
3. Replace body calls mechanically according to the decision table.
4. Remove any remaining imports of `ValueBox` and `ValueBoxPointer`.
5. Search for `.release()`, `.take_value()`, `.with_value()`, `*mut ValueBox<`, `ValueBox::new`, and deprecated helper names.
6. Reject any migration that introduces borrowing from `OwnedPtr<T>`.
7. Reject any migration that keeps ownership-taking behavior on `BorrowedPtr<T>`.

## Audit checklist

When migrating an FFI surface:

1. Change constructors to return `OwnedPtr<T>`.
2. Change read-only borrowed methods to accept `BorrowedPtr<T>`.
3. Change mutating borrowed methods to accept `mut BorrowedPtr<T>`.
4. Change destroy/consume methods to accept `OwnedPtr<T>`.
5. Replace `release()` with `drop(...)`.
6. Replace deprecated borrowed helpers with `with_ref*`, `with_mut*`, or `with_clone*`.
7. Remove any code that converts owned handles into borrowed handles.
8. Review all erased-pointer sites separately.
9. Verify that no `OwnedPtr<T>` method is used to obtain borrowed access.
