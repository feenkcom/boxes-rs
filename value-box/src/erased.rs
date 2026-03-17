use std::any::Any;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::NonNull;

use crate::{BoxerError, Result};

#[repr(transparent)]
pub struct ErasedBorrowedPtr {
    ptr: *mut c_void,
}

impl ErasedBorrowedPtr {
    /// # Safety
    ///
    /// `ptr` must be either null or point to a valid value for the duration of
    /// any pointer-based operations performed through this wrapper.
    pub const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    pub const fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub const fn as_raw(&self) -> *mut c_void {
        self.ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn with_ptr<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(NonNull<c_void>) -> Result<R>,
    {
        let pointer = NonNull::new(self.ptr)
            .ok_or_else(|| BoxerError::NullPointer("erased borrowed ptr".to_string()))?;
        op(pointer)
    }

    pub fn with_ptr_ok<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(NonNull<c_void>) -> R,
    {
        self.with_ptr(|pointer| Ok(op(pointer)))
    }
}

impl Default for ErasedBorrowedPtr {
    fn default() -> Self {
        Self::null()
    }
}

/// Tell Rust to take back the control over memory.
///
/// # Safety
///
/// `pointer` must come from [`Box::into_raw`] for the same `T`, must not be
/// null, and must not have already been reclaimed.
pub unsafe fn from_raw<T>(pointer: *mut T) -> Box<T> {
    assert!(!pointer.is_null(), "from_raw(): Pointer must not be null!");
    assert_eq!(
        size_of::<*mut T>(),
        size_of::<*mut c_void>(),
        "The pointer must be compatible with void*"
    );
    unsafe { Box::from_raw(pointer) }
}

pub fn into_raw<T>(boxed: Box<T>) -> *mut T {
    assert_eq!(
        size_of::<*mut T>(),
        size_of::<*mut c_void>(),
        "The pointer must be compatible with void*"
    );
    Box::into_raw(boxed)
}
