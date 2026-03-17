use geometry_box::PointBox;
use std::any::Any;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

pub trait BoxerPointFFI<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_point_default() -> OwnedPtr<PointBox<T>>;

    fn boxer_point_create(x: T, y: T) -> OwnedPtr<PointBox<T>>;

    fn boxer_point_drop(ptr: OwnedPtr<PointBox<T>>);

    fn boxer_point_get_x(point_box: BorrowedPtr<PointBox<T>>) -> T;

    fn boxer_point_set_x(point_box: BorrowedPtr<PointBox<T>>, x: T);

    fn boxer_point_get_y(point_box: BorrowedPtr<PointBox<T>>) -> T;

    fn boxer_point_set_y(point_box: BorrowedPtr<PointBox<T>>, y: T);
}

impl<T> BoxerPointFFI<T> for PointBox<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_point_default() -> OwnedPtr<PointBox<T>> {
        OwnedPtr::new(PointBox::<T>::default())
    }

    fn boxer_point_create(x: T, y: T) -> OwnedPtr<PointBox<T>> {
        OwnedPtr::new(PointBox::<T>::new(x, y))
    }

    fn boxer_point_drop(point_box: OwnedPtr<PointBox<T>>) {
        drop(point_box);
    }

    fn boxer_point_get_x(point_box: BorrowedPtr<PointBox<T>>) -> T {
        point_box.with_ref_ok(|point| point.x).or_log(0u8.into())
    }

    fn boxer_point_set_x(mut point_box: BorrowedPtr<PointBox<T>>, x: T) {
        point_box.with_mut_ok(|point| point.x = x).log();
    }

    fn boxer_point_get_y(point_box: BorrowedPtr<PointBox<T>>) -> T {
        point_box.with_ref_ok(|point| point.y).or_log(0u8.into())
    }

    fn boxer_point_set_y(mut point_box: BorrowedPtr<PointBox<T>>, y: T) {
        point_box.with_mut_ok(|point| point.y = y).log();
    }
}
