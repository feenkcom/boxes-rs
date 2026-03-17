use geometry_box::SizeBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::size::SizeBoxFFI;

pub type BoxerSizeI32 = SizeBox<i32>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_i32_create() -> OwnedPtr<BoxerSizeI32> {
    BoxerSizeI32::boxer_size_create()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_i32_drop(ptr: OwnedPtr<BoxerSizeI32>) {
    BoxerSizeI32::boxer_size_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_i32_get_width(ptr: BorrowedPtr<BoxerSizeI32>) -> i32 {
    BoxerSizeI32::boxer_size_get_width(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_i32_set_width(mut ptr: BorrowedPtr<BoxerSizeI32>, width: i32) {
    let _ = &mut ptr;
    BoxerSizeI32::boxer_size_set_width(ptr, width);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_i32_get_height(ptr: BorrowedPtr<BoxerSizeI32>) -> i32 {
    BoxerSizeI32::boxer_size_get_height(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_i32_set_height(mut ptr: BorrowedPtr<BoxerSizeI32>, height: i32) {
    let _ = &mut ptr;
    BoxerSizeI32::boxer_size_set_height(ptr, height);
}
