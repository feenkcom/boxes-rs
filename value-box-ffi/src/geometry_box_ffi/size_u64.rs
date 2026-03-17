use geometry_box::SizeBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::size::SizeBoxFFI;

pub type BoxerSizeU64 = SizeBox<u64>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u64_create() -> OwnedPtr<BoxerSizeU64> {
    BoxerSizeU64::boxer_size_create()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u64_drop(ptr: OwnedPtr<BoxerSizeU64>) {
    BoxerSizeU64::boxer_size_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u64_get_width(ptr: BorrowedPtr<BoxerSizeU64>) -> u64 {
    BoxerSizeU64::boxer_size_get_width(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u64_set_width(mut ptr: BorrowedPtr<BoxerSizeU64>, width: u64) {
    let _ = &mut ptr;
    BoxerSizeU64::boxer_size_set_width(ptr, width);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u64_get_height(ptr: BorrowedPtr<BoxerSizeU64>) -> u64 {
    BoxerSizeU64::boxer_size_get_height(ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_size_u64_set_height(mut ptr: BorrowedPtr<BoxerSizeU64>, height: u64) {
    let _ = &mut ptr;
    BoxerSizeU64::boxer_size_set_height(ptr, height);
}
