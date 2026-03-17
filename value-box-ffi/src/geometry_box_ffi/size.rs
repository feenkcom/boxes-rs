use geometry_box::SizeBox;
use std::any::Any;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

pub trait SizeBoxFFI<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_size_create() -> OwnedPtr<SizeBox<T>>;

    fn boxer_size_drop(ptr: OwnedPtr<SizeBox<T>>);

    fn boxer_size_get_width(_ptr: BorrowedPtr<SizeBox<T>>) -> T;

    fn boxer_size_set_width(_ptr: BorrowedPtr<SizeBox<T>>, width: T);

    fn boxer_size_get_height(_ptr: BorrowedPtr<SizeBox<T>>) -> T;

    fn boxer_size_set_height(_ptr: BorrowedPtr<SizeBox<T>>, height: T);
}

impl<T> SizeBoxFFI<T> for SizeBox<T>
where
    T: From<u8> + Default + Copy + Any,
{
    fn boxer_size_create() -> OwnedPtr<SizeBox<T>> {
        OwnedPtr::new(SizeBox::<T>::default())
    }

    fn boxer_size_drop(size: OwnedPtr<SizeBox<T>>) {
        drop(size);
    }

    fn boxer_size_get_width(size: BorrowedPtr<SizeBox<T>>) -> T {
        size.with_ref_ok(|size| size.width).or_log(0u8.into())
    }

    fn boxer_size_set_width(mut size: BorrowedPtr<SizeBox<T>>, width: T) {
        size.with_mut_ok(|size| size.width = width).log();
    }

    fn boxer_size_get_height(size: BorrowedPtr<SizeBox<T>>) -> T {
        size.with_ref_ok(|size| size.height).or_log(0u8.into())
    }

    fn boxer_size_set_height(mut size: BorrowedPtr<SizeBox<T>>, height: T) {
        size.with_mut_ok(|size| size.height = height).log();
    }
}
