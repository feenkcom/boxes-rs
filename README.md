# boxes-rs

A set of utility crates to make development of ffi bindings to Rust libraries easier:

- [`value-box`](value-box) - passing arbitrary Rust structures back and forth
- [`string-box`](string-box) - working with byte, wide and utf8-encoded strings
- [`array-box`](array-box) - create and access an array of items

Migration guide:
- [`MIGRATION_V4.md`](MIGRATION_V4.md) - migrating from the old `ValueBox` API to `OwnedPtr<T>` and `BorrowedPtr<T>`

## License

Copyright feenk gmbh.

Licensed under MIT. See [LICENSE](LICENSE).