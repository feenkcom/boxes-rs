use geometry_box::PointBox;
use value_box::ValueBox;

use crate::point::BoxerPointFFI;

pub type BoxerPointF32 = PointBox<f32>;

#[no_mangle]
pub extern "C" fn boxer_point_f32_default() -> *mut ValueBox<BoxerPointF32> {
    BoxerPointF32::boxer_point_default()
}

#[no_mangle]
pub extern "C" fn boxer_point_f32_create(x: f32, y: f32) -> *mut ValueBox<BoxerPointF32> {
    BoxerPointF32::boxer_point_create(x, y)
}

#[no_mangle]
pub extern "C" fn boxer_point_f32_drop(ptr: *mut ValueBox<BoxerPointF32>) {
    BoxerPointF32::boxer_point_drop(ptr);
}

#[no_mangle]
pub extern "C" fn boxer_point_f32_get_x(_point_ptr: *mut ValueBox<BoxerPointF32>) -> f32 {
    BoxerPointF32::boxer_point_get_x(_point_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_point_f32_set_x(_point_ptr: *mut ValueBox<BoxerPointF32>, x: f32) {
    BoxerPointF32::boxer_point_set_x(_point_ptr, x);
}

#[no_mangle]
pub extern "C" fn boxer_point_f32_get_y(_point_ptr: *mut ValueBox<BoxerPointF32>) -> f32 {
    BoxerPointF32::boxer_point_get_y(_point_ptr)
}

#[no_mangle]
pub extern "C" fn boxer_point_f32_set_y(_point_ptr: *mut ValueBox<BoxerPointF32>, y: f32) {
    BoxerPointF32::boxer_point_set_y(_point_ptr, y);
}
