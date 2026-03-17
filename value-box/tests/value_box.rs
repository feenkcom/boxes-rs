use std::error::Error;
use std::ffi::c_void;
use std::mem::size_of;
use std::rc::Rc;

use value_box::{from_raw, BorrowedPtr, ErasedBorrowedPtr, OwnedPtr, Result};

#[test]
fn value_box_size_in_memory() -> Result<()> {
    assert_eq!(size_of::<BorrowedPtr<c_void>>(), size_of::<*mut c_void>());
    assert_eq!(size_of::<OwnedPtr<c_void>>(), size_of::<*mut c_void>());
    assert_eq!(size_of::<BorrowedPtr<(u64, u64)>>(), size_of::<*mut (u64, u64)>());
    assert_eq!(size_of::<OwnedPtr<Box<dyn Error>>>(), size_of::<*mut Box<dyn Error>>());
    Ok(())
}

#[test]
fn value_box_as_ref_mut() -> Result<()> {
    let raw = Box::into_raw(Box::new(5_i32));
    let value_box_ptr = unsafe { BorrowedPtr::from_raw(raw) };
    let value = value_box_ptr.with_ref_ok(|value| *value)?;
    assert_eq!(value, 5);
    unsafe { drop(Box::from_raw(raw)) };
    Ok(())
}

#[test]
fn value_box_as_non_null() -> Result<()> {
    let raw = Box::into_raw(Box::new(5_i32));
    let value_box_ptr = unsafe { BorrowedPtr::from_raw(raw) };
    let value = value_box_ptr.with_ptr_ok(|pointer| unsafe { *pointer.as_ptr().cast::<i32>() })?;
    assert_eq!(value, 5);
    unsafe { drop(Box::from_raw(raw)) };
    Ok(())
}

#[test]
fn erased_value_box_as_non_null() -> Result<()> {
    let raw = Box::into_raw(Box::new(7_i32));
    let erased_value_box_ptr = unsafe { ErasedBorrowedPtr::from_raw(raw.cast()) };
    let value =
        erased_value_box_ptr.with_ptr_ok(|pointer| unsafe { *pointer.as_ptr().cast::<i32>() })?;
    assert_eq!(value, 7);
    unsafe { drop(Box::from_raw(raw)) };
    Ok(())
}

#[test]
fn value_box_into_erased_raw() -> Result<()> {
    let erased_value_box_ptr = unsafe { ErasedBorrowedPtr::from_raw(Box::into_raw(Box::new(9_i32)).cast()) };
    let value =
        erased_value_box_ptr.with_ptr_ok(|pointer| unsafe { *pointer.as_ptr().cast::<i32>() })?;
    assert_eq!(value, 9);
    unsafe { from_raw::<i32>(erased_value_box_ptr.as_raw().cast()) };
    Ok(())
}

#[test]
fn const_erased_value_box_as_non_null() -> Result<()> {
    let raw = Box::into_raw(Box::new(11_i32));
    let erased_value_box_ptr = unsafe { ErasedBorrowedPtr::from_raw(raw.cast()) };
    let value =
        erased_value_box_ptr.with_ptr_ok(|pointer| unsafe { *pointer.as_ptr().cast::<i32>() })?;
    assert_eq!(value, 11);
    unsafe { drop(Box::from_raw(raw)) };
    Ok(())
}

#[test]
fn borrowed_ptr_with_clone() -> Result<()> {
    let raw = Box::into_raw(Box::new(5_i32));
    let value_box_ptr = unsafe { BorrowedPtr::from_raw(raw) };
    assert!(!value_box_ptr.is_null());

    let mut result = 0;
    value_box_ptr.with_clone_ok(|value| result = value * 2)?;
    assert!(!value_box_ptr.is_null());
    assert_eq!(result, 10);
    unsafe { drop(Box::from_raw(raw)) };
    Ok(())
}

#[test]
fn borrowed_ptr_with_clone_return() -> Result<()> {
    let raw = Box::into_raw(Box::new(5_i32));
    let value_box_ptr = unsafe { BorrowedPtr::from_raw(raw) };
    assert!(!value_box_ptr.is_null());

    let result = value_box_ptr.with_clone_ok(|value| value * 2)?;
    assert!(!value_box_ptr.is_null());
    assert_eq!(result, 10);
    unsafe { drop(Box::from_raw(raw)) };
    Ok(())
}

#[test]
fn value_box_drop() {
    let value = Rc::new(42);

    let ptr = OwnedPtr::new(value.clone());
    assert_eq!(Rc::strong_count(&value), 2);
    drop(ptr);

    assert_eq!(Rc::strong_count(&value), 1);
}

#[test]
fn owned_ptr_take_value() -> Result<()> {
    let value = Rc::new(42);
    let owned = OwnedPtr::new(value.clone());

    let extracted = owned.take_value()?;
    assert_eq!(Rc::strong_count(&value), 2);
    drop(extracted);
    assert_eq!(Rc::strong_count(&value), 1);

    Ok(())
}
