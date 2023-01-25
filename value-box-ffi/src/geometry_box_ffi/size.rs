use geometry_box::SizeBox;
use std::any::Any;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

pub trait SizeBoxFFI<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_size_create() -> *mut ValueBox<SizeBox<T>>;

    fn boxer_size_drop(ptr: *mut ValueBox<SizeBox<T>>);

    fn boxer_size_get_width(_ptr: *mut ValueBox<SizeBox<T>>) -> T;

    fn boxer_size_set_width(_ptr: *mut ValueBox<SizeBox<T>>, width: T);

    fn boxer_size_get_height(_ptr: *mut ValueBox<SizeBox<T>>) -> T;

    fn boxer_size_set_height(_ptr: *mut ValueBox<SizeBox<T>>, height: T);
}

impl<T> SizeBoxFFI<T> for SizeBox<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_size_create() -> *mut ValueBox<SizeBox<T>> {
        ValueBox::new(SizeBox::<T>::default()).into_raw()
    }

    fn boxer_size_drop(size: *mut ValueBox<SizeBox<T>>) {
        size.release();
    }

    fn boxer_size_get_width(size: *mut ValueBox<SizeBox<T>>) -> T {
        size.with_ref_ok(|size| size.width).or_log(0u8.into())
    }

    fn boxer_size_set_width(size: *mut ValueBox<SizeBox<T>>, width: T) {
        size.with_mut_ok(|size| size.width = width).log();
    }

    fn boxer_size_get_height(size: *mut ValueBox<SizeBox<T>>) -> T {
        size.with_ref_ok(|size| size.height).or_log(0u8.into())
    }

    fn boxer_size_set_height(size: *mut ValueBox<SizeBox<T>>, height: T) {
        size.with_mut_ok(|size| size.height = height).log();
    }
}
