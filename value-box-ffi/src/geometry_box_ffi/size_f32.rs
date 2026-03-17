use geometry_box::SizeBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::size::SizeBoxFFI;

pub type BoxerSizeF32 = SizeBox<f32>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f32_create() -> OwnedPtr<BoxerSizeF32> {
    BoxerSizeF32::boxer_size_create()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f32_drop(ptr: OwnedPtr<BoxerSizeF32>) {
    BoxerSizeF32::boxer_size_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f32_get_width(ptr: BorrowedPtr<BoxerSizeF32>) -> f32 {
    BoxerSizeF32::boxer_size_get_width(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f32_set_width(mut ptr: BorrowedPtr<BoxerSizeF32>, width: f32) {
    let _ = &mut ptr;
    BoxerSizeF32::boxer_size_set_width(ptr, width);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f32_get_height(ptr: BorrowedPtr<BoxerSizeF32>) -> f32 {
    BoxerSizeF32::boxer_size_get_height(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f32_set_height(mut ptr: BorrowedPtr<BoxerSizeF32>, height: f32) {
    let _ = &mut ptr;
    BoxerSizeF32::boxer_size_set_height(ptr, height);
}
