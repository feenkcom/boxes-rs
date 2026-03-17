use std::any::{Any, type_name};
use std::ffi::c_void;
use std::ptr::NonNull;

use crate::erased::ErasedBorrowedPtr;
use crate::{BoxerError, Result};

#[repr(transparent)]
pub struct BorrowedPtr<T: Any> {
    ptr: *mut T,
}

impl<T: Any> BorrowedPtr<T> {
    /// # Safety
    ///
    /// `ptr` must be either null or point to a valid `T` for the duration of
    /// any borrowing operations performed through this wrapper.
    pub const unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub const fn null() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub const fn as_raw(&self) -> *mut T {
        self.ptr
    }

    /// Erase the pointee type while preserving the same borrowed-pointer contract.
    ///
    /// The returned [`ErasedBorrowedPtr`] points to the same allocation as this
    /// `BorrowedPtr<T>`, but without the concrete `T`.
    ///
    /// # Invariants
    ///
    /// - The erased pointer is valid for at most the same duration as this
    ///   `BorrowedPtr<T>`.
    /// - Erasing the type does not extend the lifetime of the underlying value.
    /// - The erased pointer does not carry ownership and must not be used after
    ///   the original borrowed pointer would no longer be valid.
    pub fn erase(&self) -> ErasedBorrowedPtr {
        unsafe { ErasedBorrowedPtr::from_raw(self.ptr.cast()) }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    pub fn with_ref<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&T) -> Result<R>,
    {
        let pointer = self.non_null()?;
        unsafe { op(pointer.as_ref()) }
    }

    pub fn with_option_ref<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(Option<&T>) -> Result<R>,
    {
        if !self.is_null() {
            self.with_ref(|value| op(Some(value)))
        } else {
            op(None)
        }
    }

    pub fn with_ref_ok<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.with_ref(|value| Ok(op(value)))
    }

    pub fn with_mut<R: Any, F>(&mut self, op: F) -> Result<R>
    where
        F: FnOnce(&mut T) -> Result<R>,
    {
        let mut pointer = self.non_null()?;
        unsafe { op(pointer.as_mut()) }
    }

    pub fn with_mut_ok<R: Any, F>(&mut self, op: F) -> Result<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.with_mut(|value| Ok(op(value)))
    }

    pub fn with_clone<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(T) -> Result<R>,
        T: Clone,
    {
        self.with_ref(|value| op(value.clone()))
    }

    pub fn with_clone_ok<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(T) -> R,
        T: Clone,
    {
        self.with_clone(|value| Ok(op(value)))
    }

    pub fn with_ptr<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(NonNull<c_void>) -> Result<R>,
    {
        let pointer = self.non_null()?.cast::<c_void>();
        op(pointer)
    }

    pub fn with_ptr_ok<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(NonNull<c_void>) -> R,
    {
        self.with_ptr(|pointer| Ok(op(pointer)))
    }

    pub fn with_ref_ref<R: Any, F, P: Any>(&self, ptr: &BorrowedPtr<P>, op: F) -> Result<R>
    where
        F: FnOnce(&T, &P) -> Result<R>,
    {
        self.with_ref(|t| ptr.with_ref(|p| op(t, p)))
    }

    pub fn with_ref_ref_ref<R: Any, F, P1: Any, P2: Any>(
        &self,
        ptr1: &BorrowedPtr<P1>,
        ptr2: &BorrowedPtr<P2>,
        op: F,
    ) -> Result<R>
    where
        F: FnOnce(&T, &P1, &P2) -> Result<R>,
    {
        self.with_ref(|t| ptr1.with_ref(|p1| ptr2.with_ref(|p2| op(t, p1, p2))))
    }

    pub fn with_ref_ref_ref_ref<R: Any, F, P1: Any, P2: Any, P3: Any>(
        &self,
        ptr1: &BorrowedPtr<P1>,
        ptr2: &BorrowedPtr<P2>,
        ptr3: &BorrowedPtr<P3>,
        op: F,
    ) -> Result<R>
    where
        F: FnOnce(&T, &P1, &P2, &P3) -> Result<R>,
    {
        self.with_ref(|t| {
            ptr1.with_ref(|p1| ptr2.with_ref(|p2| ptr3.with_ref(|p3| op(t, p1, p2, p3))))
        })
    }

    fn non_null(&self) -> Result<NonNull<T>> {
        NonNull::new(self.ptr).ok_or_else(|| BoxerError::NullPointer(type_name::<T>().to_string()))
    }
}

impl<T: Any> Default for BorrowedPtr<T> {
    fn default() -> Self {
        Self::null()
    }
}
