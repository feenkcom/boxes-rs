use geometry_box::PointBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::point::BoxerPointFFI;

pub type BoxerPointI32 = PointBox<i32>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_i32_default() -> OwnedPtr<BoxerPointI32> {
    BoxerPointI32::boxer_point_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_i32_create(x: i32, y: i32) -> OwnedPtr<BoxerPointI32> {
    BoxerPointI32::boxer_point_create(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_i32_drop(ptr: OwnedPtr<BoxerPointI32>) {
    BoxerPointI32::boxer_point_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_i32_get_x(point_ptr: BorrowedPtr<BoxerPointI32>) -> i32 {
    BoxerPointI32::boxer_point_get_x(point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_i32_set_x(mut point_ptr: BorrowedPtr<BoxerPointI32>, x: i32) {
    let _ = &mut point_ptr;
    BoxerPointI32::boxer_point_set_x(point_ptr, x);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_i32_get_y(point_ptr: BorrowedPtr<BoxerPointI32>) -> i32 {
    BoxerPointI32::boxer_point_get_y(point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_i32_set_y(mut _point_ptr: BorrowedPtr<BoxerPointI32>, y: i32) {
    let _ = &mut _point_ptr;
    BoxerPointI32::boxer_point_set_y(_point_ptr, y);
}
