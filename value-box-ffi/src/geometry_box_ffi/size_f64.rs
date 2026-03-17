use geometry_box::SizeBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::size::SizeBoxFFI;

pub type BoxerSizeF64 = SizeBox<f64>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f64_create() -> OwnedPtr<BoxerSizeF64> {
    BoxerSizeF64::boxer_size_create()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f64_drop(ptr: OwnedPtr<BoxerSizeF64>) {
    BoxerSizeF64::boxer_size_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f64_get_width(ptr: BorrowedPtr<BoxerSizeF64>) -> f64 {
    BoxerSizeF64::boxer_size_get_width(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f64_set_width(mut ptr: BorrowedPtr<BoxerSizeF64>, width: f64) {
    let _ = &mut ptr;
    BoxerSizeF64::boxer_size_set_width(ptr, width);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f64_get_height(ptr: BorrowedPtr<BoxerSizeF64>) -> f64 {
    BoxerSizeF64::boxer_size_get_height(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_f64_set_height(mut ptr: BorrowedPtr<BoxerSizeF64>, height: f64) {
    let _ = &mut ptr;
    BoxerSizeF64::boxer_size_set_height(ptr, height);
}
