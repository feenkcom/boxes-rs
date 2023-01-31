pub mod array;

use crate::array_ffi;

array_ffi!(u8);
array_ffi!(i8);
array_ffi!(u16);
array_ffi!(i16);
array_ffi!(u32);
array_ffi!(i32);
array_ffi!(u64);
array_ffi!(i64);

array_ffi!(isize);
array_ffi!(usize);

array_ffi!(std::ffi::c_int, int);
array_ffi!(std::ffi::c_uint, uint);

array_ffi!(f32);
array_ffi!(f64);