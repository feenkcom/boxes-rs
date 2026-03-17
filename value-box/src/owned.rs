use std::any::{Any, type_name};
use std::mem::ManuallyDrop;

use crate::erased::{from_raw, into_raw};
use crate::{BoxerError, Result};

#[must_use]
#[repr(transparent)]
pub struct OwnedPtr<T: Any> {
    ptr: *mut T,
}

impl<T: Any> OwnedPtr<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: into_raw(Box::new(value)),
        }
    }

    /// # Safety
    ///
    /// `ptr` must be uniquely owned, created by `Box::into_raw(Box<T>)` for the
    /// same `T`, and must not have already been reclaimed.
    pub const unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub const fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn with_value<R: Any, F>(self, op: F) -> Result<R>
    where
        F: FnOnce(T) -> Result<R>,
    {
        self.into_value().and_then(op)
    }

    pub fn with_value_ok<R: Any, F>(self, op: F) -> Result<R>
    where
        F: FnOnce(T) -> R,
    {
        self.with_value(|value| Ok(op(value)))
    }

    fn into_value(self) -> Result<T> {
        let pointer = self.into_ptr();
        if pointer.is_null() {
            return BoxerError::NullPointer(type_name::<T>().to_string()).into();
        }

        Ok(unsafe { *from_raw(pointer) })
    }

    fn into_ptr(self) -> *mut T {
        let this = ManuallyDrop::new(self);
        this.ptr
    }
}

impl<T: Any> Default for OwnedPtr<T> {
    fn default() -> Self {
        Self::null()
    }
}

impl<T: Any> From<T> for OwnedPtr<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Any> Drop for OwnedPtr<T> {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }

        unsafe {
            drop(from_raw(self.ptr));
        }
    }
}
