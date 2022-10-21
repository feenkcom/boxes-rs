# StringBox
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/string-box.svg
[crates-url]: https://crates.io/crates/string-box
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/feenkcom/boxes-rs/blob/main/LICENSE

Allows developers to create Rust `String` from byte-string, utf-8 encoded C-string or wide-string.

## Examples

From a **not** null-terminated byte-string:
```rust
let byte_string = vec![104u8, 101, 108, 108, 111];
let string = StringBox::from_byte_string(byte_string);
assert_eq!(string.to_string(), String::from("hello"));
```

From a null-terminated utf-8 string:
```rust
let utf8_string = vec![104u8, 101, 108, 108, 111, 0];
let string = StringBox::from_utf8_string(utf8_string.as_slice());
assert_eq!(string.to_string(), String::from("hello"));
```