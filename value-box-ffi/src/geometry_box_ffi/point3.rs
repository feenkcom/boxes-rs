use geometry_box::Point3Box;
use std::any::Any;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

pub trait Point3BoxFFI<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_point_default() -> OwnedPtr<Point3Box<T>>;

    fn boxer_point_create(x: T, y: T, z: T) -> OwnedPtr<Point3Box<T>>;

    fn boxer_point_drop(ptr: OwnedPtr<Point3Box<T>>);

    fn boxer_point_get_x(_maybe_null_ptr: BorrowedPtr<Point3Box<T>>) -> T;

    fn boxer_point_set_x(_maybe_null_ptr: BorrowedPtr<Point3Box<T>>, x: T);

    fn boxer_point_get_y(_maybe_null_ptr: BorrowedPtr<Point3Box<T>>) -> T;

    fn boxer_point_set_y(_maybe_null_ptr: BorrowedPtr<Point3Box<T>>, y: T);

    fn boxer_point_get_z(_maybe_null_ptr: BorrowedPtr<Point3Box<T>>) -> T;

    fn boxer_point_set_z(_maybe_null_ptr: BorrowedPtr<Point3Box<T>>, z: T);
}

impl<T> Point3BoxFFI<T> for Point3Box<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_point_default() -> OwnedPtr<Point3Box<T>> {
        OwnedPtr::new(Point3Box::<T>::default())
    }

    fn boxer_point_create(x: T, y: T, z: T) -> OwnedPtr<Point3Box<T>> {
        OwnedPtr::new(Point3Box::<T>::new(x, y, z))
    }

    fn boxer_point_drop(ptr: OwnedPtr<Point3Box<T>>) {
        drop(ptr);
    }

    fn boxer_point_get_x(point: BorrowedPtr<Point3Box<T>>) -> T {
        point.with_ref_ok(|point| point.x).or_log(0u8.into())
    }

    fn boxer_point_set_x(mut point: BorrowedPtr<Point3Box<T>>, x: T) {
        point.with_mut_ok(|point| point.x = x).log()
    }

    fn boxer_point_get_y(point: BorrowedPtr<Point3Box<T>>) -> T {
        point.with_ref_ok(|point| point.y).or_log(0u8.into())
    }

    fn boxer_point_set_y(mut point: BorrowedPtr<Point3Box<T>>, y: T) {
        point.with_mut_ok(|point| point.y = y).log();
    }

    fn boxer_point_get_z(point: BorrowedPtr<Point3Box<T>>) -> T {
        point.with_ref_ok(|point| point.z).or_log(0u8.into())
    }

    fn boxer_point_set_z(mut point: BorrowedPtr<Point3Box<T>>, z: T) {
        point.with_mut_ok(|point| point.z = z).log();
    }
}
