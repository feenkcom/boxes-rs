use std::any::Any;

use array_box::ArrayBox;

use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

pub trait ArrayBoxFFI<T>
where
    T: Default + Copy + Any,
{
    fn boxer_array_byte_size(count: usize) -> usize;
    fn boxer_array_create() -> *mut ValueBox<ArrayBox<T>>;
    fn boxer_array_create_with(element: T, amount: usize) -> *mut ValueBox<ArrayBox<T>>;

    fn boxer_array_create_from_data(_data: *mut T, amount: usize) -> *mut ValueBox<ArrayBox<T>>;

    fn boxer_array_drop(ptr: *mut ValueBox<ArrayBox<T>>);

    fn boxer_array_copy_into(
        _maybe_null_source_ptr: *mut ValueBox<ArrayBox<T>>,
        _maybe_null_destination_ptr: *mut ValueBox<ArrayBox<T>>,
    );

    fn boxer_array_copy_into_data(
        _maybe_null_source_ptr: *mut ValueBox<ArrayBox<T>>,
        _destination_data: *mut T,
        length: usize,
    );

    fn boxer_array_get_length(_maybe_null_ptr: *mut ValueBox<ArrayBox<T>>) -> usize;

    fn boxer_array_get_capacity(_maybe_null_ptr: *mut ValueBox<ArrayBox<T>>) -> usize;

    fn boxer_array_get_data(_maybe_null_ptr: *mut ValueBox<ArrayBox<T>>) -> *mut T;

    fn boxer_array_at_put(_maybe_null_ptr: *mut ValueBox<ArrayBox<T>>, index: usize, item: T)
    where
        T: Clone;

    fn boxer_array_at(_maybe_null_ptr: *mut ValueBox<ArrayBox<T>>, index: usize, default: T) -> T
    where
        T: Clone;
}

impl<T> ArrayBoxFFI<T> for ArrayBox<T>
where
    T: Default + Copy + Any,
{
    fn boxer_array_byte_size(count: usize) -> usize {
        std::mem::size_of::<T>() * count
    }

    fn boxer_array_create() -> *mut ValueBox<ArrayBox<T>> {
        ValueBox::new(ArrayBox::<T>::default()).into_raw()
    }

    fn boxer_array_create_with(element: T, amount: usize) -> *mut ValueBox<ArrayBox<T>> {
        ValueBox::new(ArrayBox::<T>::from_vector(vec![element; amount])).into_raw()
    }

    fn boxer_array_create_from_data(_data: *mut T, amount: usize) -> *mut ValueBox<ArrayBox<T>> {
        ValueBox::new(ArrayBox::<T>::from_data(_data, amount)).into_raw()
    }

    fn boxer_array_drop(ptr: *mut ValueBox<ArrayBox<T>>) {
        ptr.release();
    }

    fn boxer_array_copy_into(
        source_array: *mut ValueBox<ArrayBox<T>>,
        destination_array: *mut ValueBox<ArrayBox<T>>,
    ) {
        source_array
            .with_ref(|source_array| {
                destination_array.with_mut_ok(|destination_array| {
                    source_array.copy_into(destination_array);
                })
            })
            .log();
    }

    fn boxer_array_copy_into_data(
        source_array: *mut ValueBox<ArrayBox<T>>,
        destination_data: *mut T,
        length: usize,
    ) {
        source_array
            .with_ref(|source_array| {
                if source_array.length > length {
                    BoxerError::AnyError(
                        format!(
                            "The source (len = {}) does not fit into destination (len = {})",
                            source_array.length, length
                        )
                        .into(),
                    )
                    .into()
                } else if source_array.data.is_null() {
                    BoxerError::AnyError("The source data must not be nil".into()).into()
                } else if destination_data.is_null() {
                    BoxerError::AnyError("The destination data must not be nil".into()).into()
                } else {
                    Ok(unsafe {
                        std::ptr::copy_nonoverlapping::<T>(
                            source_array.data,
                            destination_data,
                            length,
                        )
                    })
                }
            })
            .log();
    }

    fn boxer_array_get_length(array_box: *mut ValueBox<ArrayBox<T>>) -> usize {
        array_box.with_ref_ok(|array| array.length).or_log(0)
    }

    fn boxer_array_get_capacity(array_box: *mut ValueBox<ArrayBox<T>>) -> usize {
        array_box.with_ref_ok(|array| array.capacity).or_log(0)
    }

    fn boxer_array_get_data(array_box: *mut ValueBox<ArrayBox<T>>) -> *mut T {
        array_box
            .with_ref_ok(|array| array.data)
            .or_log(std::ptr::null_mut())
    }

    fn boxer_array_at_put(array_box: *mut ValueBox<ArrayBox<T>>, index: usize, item: T)
    where
        T: Clone,
    {
        array_box
            .with_mut_ok(|array| array.at_put(index, item))
            .log();
    }

    fn boxer_array_at(array_box: *mut ValueBox<ArrayBox<T>>, index: usize, default: T) -> T
    where
        T: Clone,
    {
        array_box
            .with_ref_ok(|array| array.at(index))
            .or_log(default)
    }
}
