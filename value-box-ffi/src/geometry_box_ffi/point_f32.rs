use geometry_box::PointBox;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::point::BoxerPointFFI;

pub type BoxerPointF32 = PointBox<f32>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f32_default() -> OwnedPtr<BoxerPointF32> {
    BoxerPointF32::boxer_point_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f32_create(x: f32, y: f32) -> OwnedPtr<BoxerPointF32> {
    BoxerPointF32::boxer_point_create(x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f32_drop(ptr: OwnedPtr<BoxerPointF32>) {
    BoxerPointF32::boxer_point_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f32_get_x(_point_ptr: BorrowedPtr<BoxerPointF32>) -> f32 {
    BoxerPointF32::boxer_point_get_x(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f32_set_x(mut _point_ptr: BorrowedPtr<BoxerPointF32>, x: f32) {
    let _ = &mut _point_ptr;
    BoxerPointF32::boxer_point_set_x(_point_ptr, x);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f32_get_y(_point_ptr: BorrowedPtr<BoxerPointF32>) -> f32 {
    BoxerPointF32::boxer_point_get_y(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point_f32_set_y(mut _point_ptr: BorrowedPtr<BoxerPointF32>, y: f32) {
    let _ = &mut _point_ptr;
    BoxerPointF32::boxer_point_set_y(_point_ptr, y);
}
