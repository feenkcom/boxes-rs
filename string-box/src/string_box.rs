use std::ffi::CStr;
use std::ops::Range;
use std::slice;
use widestring::U32String;

#[derive(Debug, Clone)]
#[repr(C)]
pub enum StringBoxOrigin {
    Byte(Vec<u8>),
    String,
    Wide(U32String),
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum StringBoxOriginType {
    Byte,
    UTF8,
    Wide,
}

impl From<StringBoxOrigin> for StringBoxOriginType {
    fn from(origin: StringBoxOrigin) -> StringBoxOriginType {
        match origin {
            StringBoxOrigin::Byte(_) => StringBoxOriginType::Byte,
            StringBoxOrigin::String => StringBoxOriginType::UTF8,
            StringBoxOrigin::Wide(_) => StringBoxOriginType::Wide,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringBox {
    origin: StringBoxOrigin,
    string: String,
}

impl StringBox {
    pub fn new() -> Self {
        Self::from_string(String::new())
    }

    /// Create from Rust string
    pub fn from_string(string: String) -> Self {
        Self {
            origin: StringBoxOrigin::String,
            string,
        }
    }

    /// Create from a wide string by copying the data
    pub unsafe fn from_wide_string_data(data: *const u32, length: usize) -> Self {
        let wide_string = slice::from_raw_parts(data, length).to_vec();
        Self::from_wide_string(wide_string)
    }

    /// Create from a wide string vector
    pub fn from_wide_string(data: Vec<u32>) -> Self {
        let wide_string = U32String::from_vec(data);
        let string = wide_string.to_string_lossy();
        Self {
            origin: StringBoxOrigin::Wide(wide_string),
            string,
        }
    }

    /// Create from a wide string by copying the data
    pub unsafe fn from_byte_string_data(data: *const u8, length: usize) -> Self {
        let byte_string = slice::from_raw_parts(data, length).to_vec();
        Self::from_byte_string(byte_string)
    }

    /// Create from a byte string vector
    pub fn from_byte_string(data: Vec<u8>) -> Self {
        let string = data.iter().map(|&c| c as char).collect::<String>();
        Self {
            origin: StringBoxOrigin::Byte(data),
            string,
        }
    }

    /// data must be nul terminated
    /// length does not take nul into account
    pub unsafe fn from_utf8_string_data(data: *const u8, length: usize) -> Self {
        // we are not using CString::from_raw because it retakes the ownership
        // and will drop the data. Instead we create a borrowed CStr which
        // we later copy into a String
        Self::from_utf8_string(slice::from_raw_parts(data, length + 1))
    }

    /// data must be nul terminated
    /// length does not take nul into account
    pub fn from_utf8_string(data: &[u8]) -> Self {
        let string = unsafe {
            CStr::from_bytes_with_nul_unchecked(data)
                .to_string_lossy()
                .into_owned()
        };
        Self {
            origin: StringBoxOrigin::String,
            string,
        }
    }

    /// Replace the string with a given instance
    pub fn set_string(&mut self, string: String) {
        self.origin = StringBoxOrigin::String;
        self.string = string;
    }

    /// Returns the length of this `String`, in bytes, not [`char`]s or
    /// graphemes. In other words, it may not be what a human considers the
    /// length of the string.
    pub fn len(&self) -> usize {
        self.string.len()
    }

    /// Returns the amount of [`char`]
    pub fn char_count(&self) -> usize {
        self.string.chars().count()
    }

    pub fn to_string(&self) -> String {
        self.string.clone()
    }

    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.string.as_ptr()
    }

    pub fn char_index_to_byte_range(&self, index: usize) -> Range<usize> {
        let mut current_char_index = 0 as usize;
        let mut previous_byte_offset = 0 as usize;

        for (current_byte_offset, _) in self.string.char_indices() {
            if current_char_index == (index + 1) {
                return previous_byte_offset..current_byte_offset;
            }
            current_char_index = current_char_index + 1;
            previous_byte_offset = current_byte_offset;
        }
        previous_byte_offset..self.len()
    }

    pub fn char_index_to_utf16_range(&self, index: usize) -> Range<usize> {
        let mut current_char_index = 0 as usize;
        let mut previous_byte_offset = 0 as usize;
        let mut previous_utf16_offset = 0 as usize;

        for (current_byte_offset, _) in self.string.char_indices() {
            let delta = ((current_byte_offset - previous_byte_offset) + 1) / 2;
            if current_char_index == (index + 1) {
                return previous_utf16_offset..(previous_utf16_offset + delta);
            }
            current_char_index = current_char_index + 1;
            previous_byte_offset = current_byte_offset;
            previous_utf16_offset = previous_utf16_offset + delta;
        }
        let delta = ((self.len() - previous_byte_offset) + 1) / 2;
        previous_utf16_offset..(previous_utf16_offset + delta)
    }

    pub fn utf16_position_to_char_index(&self, index: usize) -> usize {
        let mut current_char_index = 0 as usize;
        let mut previous_byte_offset = 0 as usize;
        let mut previous_utf16_offset = 0 as usize;

        for (current_byte_offset, _) in self.string.char_indices() {
            let delta = ((current_byte_offset - previous_byte_offset) + 1) / 2;
            let current_utf16_offset = previous_utf16_offset + delta;

            if current_utf16_offset >= index {
                return current_char_index;
            }

            current_char_index = current_char_index + 1;
            previous_byte_offset = current_byte_offset;
            previous_utf16_offset = current_utf16_offset;
        }
        current_char_index
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_from_wide_string() {
        let wide_string = vec![1087u32, 1088, 1080, 1074, 1077, 1090];
        let string = StringBox::from_wide_string(wide_string);

        assert_eq!(string.to_string(), String::from("привет"));
    }

    #[test]
    pub fn test_from_byte_string() {
        let byte_string = vec![104u8, 101, 108, 108, 111];
        let string = StringBox::from_byte_string(byte_string);

        assert_eq!(string.to_string(), String::from("hello"));
    }

    #[test]
    pub fn test_from_utf8_string() {
        let utf8_string = vec![104u8, 101, 108, 108, 111, 0];
        let string = StringBox::from_utf8_string(utf8_string.as_slice());

        assert_eq!(string.to_string(), String::from("hello"));
    }

    #[test]
    pub fn sparkle() {
        let sparkle = String::from("💖");
        let string_box = StringBox::from_string(sparkle.clone());

        assert_eq!(sparkle.len(), 4);
        assert_eq!(string_box.len(), 4);
        assert_eq!(string_box.char_count(), 1);

        for char in sparkle.char_indices() {
            println!("{:?}", char);
        }
        println!("{:?}", sparkle.bytes());
    }
}
