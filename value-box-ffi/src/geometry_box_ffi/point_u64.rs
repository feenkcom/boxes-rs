use geometry_box::PointBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::point::BoxerPointFFI;

pub type BoxerPointU64 = PointBox<u64>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_u64_default() -> OwnedPtr<BoxerPointU64> {
    BoxerPointU64::boxer_point_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_u64_create(x: u64, y: u64) -> OwnedPtr<BoxerPointU64> {
    BoxerPointU64::boxer_point_create(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_u64_drop(ptr: OwnedPtr<BoxerPointU64>) {
    BoxerPointU64::boxer_point_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_u64_get_x(_point_ptr: BorrowedPtr<BoxerPointU64>) -> u64 {
    BoxerPointU64::boxer_point_get_x(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_u64_set_x(mut _point_ptr: BorrowedPtr<BoxerPointU64>, x: u64) {
    let _ = &mut _point_ptr;
    BoxerPointU64::boxer_point_set_x(_point_ptr, x);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_u64_get_y(_point_ptr: BorrowedPtr<BoxerPointU64>) -> u64 {
    BoxerPointU64::boxer_point_get_y(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_u64_set_y(mut _point_ptr: BorrowedPtr<BoxerPointU64>, y: u64) {
    let _ = &mut _point_ptr;
    BoxerPointU64::boxer_point_set_y(_point_ptr, y);
}
