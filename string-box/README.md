# StringBox

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