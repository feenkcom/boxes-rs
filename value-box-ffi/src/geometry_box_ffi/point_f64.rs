use geometry_box::PointBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::point::BoxerPointFFI;

pub type BoxerPointF64 = PointBox<f64>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f64_default() -> OwnedPtr<BoxerPointF64> {
    BoxerPointF64::boxer_point_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f64_create(x: f64, y: f64) -> OwnedPtr<BoxerPointF64> {
    BoxerPointF64::boxer_point_create(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f64_drop(ptr: OwnedPtr<BoxerPointF64>) {
    BoxerPointF64::boxer_point_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f64_get_x(_point_ptr: BorrowedPtr<BoxerPointF64>) -> f64 {
    BoxerPointF64::boxer_point_get_x(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f64_set_x(mut _point_ptr: BorrowedPtr<BoxerPointF64>, x: f64) {
    let _ = &mut _point_ptr;
    BoxerPointF64::boxer_point_set_x(_point_ptr, x);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f64_get_y(_point_ptr: BorrowedPtr<BoxerPointF64>) -> f64 {
    BoxerPointF64::boxer_point_get_y(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f64_set_y(mut _point_ptr: BorrowedPtr<BoxerPointF64>, y: f64) {
    let _ = &mut _point_ptr;
    BoxerPointF64::boxer_point_set_y(_point_ptr, y);
}
