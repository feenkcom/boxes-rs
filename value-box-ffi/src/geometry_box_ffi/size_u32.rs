use geometry_box::SizeBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::size::SizeBoxFFI;

pub type BoxerSizeU32 = SizeBox<u32>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u32_create() -> OwnedPtr<BoxerSizeU32> {
    BoxerSizeU32::boxer_size_create()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u32_drop(ptr: OwnedPtr<BoxerSizeU32>) {
    BoxerSizeU32::boxer_size_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u32_get_width(ptr: BorrowedPtr<BoxerSizeU32>) -> u32 {
    BoxerSizeU32::boxer_size_get_width(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u32_set_width(mut ptr: BorrowedPtr<BoxerSizeU32>, width: u32) {
    let _ = &mut ptr;
    BoxerSizeU32::boxer_size_set_width(ptr, width);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u32_get_height(ptr: BorrowedPtr<BoxerSizeU32>) -> u32 {
    BoxerSizeU32::boxer_size_get_height(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u32_set_height(mut ptr: BorrowedPtr<BoxerSizeU32>, height: u32) {
    let _ = &mut ptr;
    BoxerSizeU32::boxer_size_set_height(ptr, height);
}
