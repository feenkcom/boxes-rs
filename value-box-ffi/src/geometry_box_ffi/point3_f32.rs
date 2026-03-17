use geometry_box::Point3Box;
use value_box::{BorrowedPtr, OwnedPtr};

use crate::point3::Point3BoxFFI;

pub type BoxerPoint3F32 = Point3Box<f32>;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_default() -> OwnedPtr<BoxerPoint3F32> {
    BoxerPoint3F32::boxer_point_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_create(x: f32, y: f32, z: f32) -> OwnedPtr<BoxerPoint3F32> {
    BoxerPoint3F32::boxer_point_create(x, y, z)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_drop(ptr: OwnedPtr<BoxerPoint3F32>) {
    BoxerPoint3F32::boxer_point_drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_get_x(_point_ptr: BorrowedPtr<BoxerPoint3F32>) -> f32 {
    BoxerPoint3F32::boxer_point_get_x(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_set_x(mut _point_ptr: BorrowedPtr<BoxerPoint3F32>, x: f32) {
    let _ = &mut _point_ptr;
    BoxerPoint3F32::boxer_point_set_x(_point_ptr, x);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_get_y(_point_ptr: BorrowedPtr<BoxerPoint3F32>) -> f32 {
    BoxerPoint3F32::boxer_point_get_y(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_set_y(mut _point_ptr: BorrowedPtr<BoxerPoint3F32>, y: f32) {
    let _ = &mut _point_ptr;
    BoxerPoint3F32::boxer_point_set_y(_point_ptr, y);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_get_z(_point_ptr: BorrowedPtr<BoxerPoint3F32>) -> f32 {
    BoxerPoint3F32::boxer_point_get_z(_point_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_point3_f32_set_z(mut _point_ptr: BorrowedPtr<BoxerPoint3F32>, z: f32) {
    let _ = &mut _point_ptr;
    BoxerPoint3F32::boxer_point_set_z(_point_ptr, z);
}
