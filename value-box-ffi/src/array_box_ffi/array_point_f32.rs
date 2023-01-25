use array_box::ArrayBox;
use geometry_box::PointBox;

use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use crate::array::ArrayBoxFFI;

pub type BoxerPointF32 = PointBox<f32>;
pub type BoxerArrayPointF32 = ArrayBox<BoxerPointF32>;

#[no_mangle]
pub extern "C" fn boxer_array_point_f32_create() -> *mut ValueBox<BoxerArrayPointF32> {
    BoxerArrayPointF32::boxer_array_create()
}

#[no_mangle]
pub extern "C" fn boxer_array_point_f32_create_with(
    point: *mut ValueBox<BoxerPointF32>,
    amount: usize,
) -> *mut ValueBox<BoxerArrayPointF32> {
    point
        .with_clone_ok(|point| BoxerArrayPointF32::boxer_array_create_with(point, amount))
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn boxer_array_point_f32_create_from_data(
    data: *mut BoxerPointF32,
    amount: usize,
) -> *mut ValueBox<BoxerArrayPointF32> {
    BoxerArrayPointF32::boxer_array_create_from_data(data, amount)
}

#[no_mangle]
pub extern "C" fn boxer_array_point_f32_drop(array: *mut ValueBox<BoxerArrayPointF32>) {
    BoxerArrayPointF32::boxer_array_drop(array);
}

#[no_mangle]
pub extern "C" fn boxer_array_point_f32_get_length(
    array: *mut ValueBox<BoxerArrayPointF32>,
) -> usize {
    BoxerArrayPointF32::boxer_array_get_length(array)
}

#[no_mangle]
pub extern "C" fn boxer_array_point_f32_get_capacity(
    array: *mut ValueBox<BoxerArrayPointF32>,
) -> usize {
    BoxerArrayPointF32::boxer_array_get_capacity(array)
}

#[no_mangle]
pub extern "C" fn boxer_array_point_f32_get_data(
    array: *mut ValueBox<BoxerArrayPointF32>,
) -> *mut BoxerPointF32 {
    BoxerArrayPointF32::boxer_array_get_data(array)
}

#[cfg(test)]
mod test {
    use crate::array_point_f32::{
        boxer_array_point_f32_create_with, boxer_array_point_f32_drop,
        boxer_array_point_f32_get_length,
    };
    use crate::point_f32::{
        boxer_point_f32_default, boxer_point_f32_drop, boxer_point_f32_get_x, boxer_point_f32_get_y,
    };

    #[test]
    fn create_with_point() {
        let point = boxer_point_f32_default();
        assert_eq!(boxer_point_f32_get_x(point), 0.0);
        assert_eq!(boxer_point_f32_get_y(point), 0.0);

        let array = boxer_array_point_f32_create_with(point, 10);
        // array is created with a clone of the point, we can drop the point
        boxer_point_f32_drop(point);

        assert_eq!(boxer_array_point_f32_get_length(array), 10);
        boxer_array_point_f32_drop(array);
    }
}
