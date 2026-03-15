use std::any::{type_name, Any};
use std::mem::size_of;

use crate::{BoxerError, Result, ReturnBoxerResult};

#[repr(transparent)]
pub struct ValueBox<T: Any> {
    value: T,
}

impl<T: Any> ValueBox<T> {
    pub fn new(object: T) -> Self {
        Self { value: object }
    }

    pub fn has_value(&self) -> bool {
        true
    }

    pub fn replace_value(&mut self, object: T) -> T {
        std::mem::replace(&mut self.value, object)
    }

    pub fn set_value(&mut self, object: T) {
        self.value = object;
    }

    pub fn clone_value(&self) -> T
    where
        T: Clone,
    {
        self.value.clone()
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn into_raw(self) -> *mut Self {
        let ptr: *mut T = into_raw(Box::new(self.value));
        ptr.cast()
    }
}

impl<T: Any> AsRef<T> for ValueBox<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T: Any> AsMut<T> for ValueBox<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

pub trait ValueBoxPointer<T: Any> {
    /// Take the value out of the box.
    fn take_value(&self) -> Result<T>;

    /// Evaluate a given function with a reference to the boxed value.
    /// The reference can not outlive the closure.
    fn with_ref<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&T) -> Result<R>;

    /// Try to unbox the value and evaluate a given function with either Some
    /// if the value was there or None if the pointer was a null pointer.
    /// Returns an error if the value box wasn't a null pointer, but the boxed value
    /// was already taken or of the wrong type.
    fn with_option_ref<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(Option<&T>) -> Result<R>,
    {
        if self.has_value() {
            self.with_ref(|value| op(Some(value)))
        } else {
            op(None)
        }
    }

    /// Evaluate a given function with a reference to the boxed value.
    /// The the reference can not outlive the closure.
    fn with_ref_ok<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.with_ref(|value| Ok(op(value)))
    }

    /// Evaluate a given function with a mutable reference to the boxed value.
    /// The lifetime of the reference can not outlive the closure.
    fn with_mut<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&mut T) -> Result<R>;

    /// Evaluate a given function that can not fail with a mutable reference to the boxed value.
    /// The lifetime of the reference can not outlive the closure.
    fn with_mut_ok<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.with_mut(|value| Ok(op(value)))
    }

    /// Evaluate a given function with a clone of the boxed value.
    /// The boxed type `T` must implement [`Clone`].
    fn with_clone<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(T) -> Result<R>,
        T: Clone,
    {
        self.with_ref(|value| op(value.clone()))
    }

    /// Evaluate a given function with a clone of the boxed value.
    /// The boxed type `T` must implement [`Clone`].
    fn with_clone_ok<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(T) -> R,
        T: Clone,
    {
        self.with_clone(|value| Ok(op(value)))
    }

    /// Evaluate a given function with references to given boxed values.
    /// The lifetime of the reference can not outlive the closure.
    fn with_ref_ref<R: Any, F, P: Any>(&self, ptr: *mut ValueBox<P>, op: F) -> Result<R>
    where
        F: FnOnce(&T, &P) -> Result<R>,
    {
        self.with_ref(|t| ptr.with_ref(|p| op(t, p)))
    }

    /// Evaluate a given function with references to given boxed values.
    /// The lifetime of the reference can not outlive the closure.
    fn with_ref_ref_ref<R: Any, F, P1: Any, P2: Any>(
        &self,
        ptr1: *mut ValueBox<P1>,
        ptr2: *mut ValueBox<P2>,
        op: F,
    ) -> Result<R>
    where
        F: FnOnce(&T, &P1, &P2) -> Result<R>,
    {
        self.with_ref(|t| ptr1.with_ref(|p1| ptr2.with_ref(|p2| op(t, p1, p2))))
    }

    /// Evaluate a given function with references to given boxed values.
    /// The lifetime of the reference can not outlive the closure.
    fn with_ref_ref_ref_ref<R: Any, F, P1: Any, P2: Any, P3: Any>(
        &self,
        ptr1: *mut ValueBox<P1>,
        ptr2: *mut ValueBox<P2>,
        ptr3: *mut ValueBox<P3>,
        op: F,
    ) -> Result<R>
    where
        F: FnOnce(&T, &P1, &P2, &P3) -> Result<R>,
    {
        self.with_ref(|t| {
            ptr1.with_ref(|p1| ptr2.with_ref(|p2| ptr3.with_ref(|p3| op(t, p1, p2, p3))))
        })
    }

    /// Evaluate a given function with the value taken out of the box
    /// and place the new value back. The value returned by the function
    /// must be of the same type as the box
    fn replace_value<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(T) -> T;

    fn release(self);

    fn has_value(&self) -> bool {
        self.with_ref_ok(|_| ()).is_ok()
    }

    #[deprecated(since = "0.1.0", note = "please use `has_value` instead")]
    fn is_valid(&self) -> bool {
        self.has_value()
    }

    #[deprecated(since = "0.1.0", note = "please use `with_ref` or `with_mut` instead")]
    fn with_not_null<Block>(&self, block: Block)
    where
        Block: FnOnce(&mut T),
    {
        self.with_mut_ok(|value| block(value)).log();
    }

    #[deprecated(since = "0.1.0", note = "please use `with_ref` or `with_mut` instead")]
    fn with_not_null_return<Block, Return: Any>(&self, default: Return, block: Block) -> Return
    where
        Block: FnOnce(&mut T) -> Return,
    {
        self.with_mut_ok(|value| block(value)).or_log(default)
    }

    #[deprecated(since = "0.1.0", note = "please use `with_ref` or `with_mut` instead")]
    fn with_value<DefaultBlock, Block, Return: Any>(
        &self,
        default: DefaultBlock,
        block: Block,
    ) -> Return
    where
        DefaultBlock: FnOnce() -> Return,
        Block: FnOnce(T) -> Return,
        T: Clone,
    {
        self.with_clone_ok(block).unwrap_or_else(|_| default())
    }

    #[deprecated(since = "0.1.0", note = "please use `with_ref` or `with_mut` instead")]
    fn with_not_null_value<Block>(&self, block: Block)
    where
        Block: FnOnce(T),
        T: Clone,
    {
        self.with_ref_ok(|value| block(value.clone())).log();
    }

    #[deprecated(since = "0.1.0", note = "please use `with_ref` or `with_mut` instead")]
    fn with_not_null_value_return<Block, Return: Any>(
        &self,
        default: Return,
        block: Block,
    ) -> Return
    where
        Block: FnOnce(T) -> Return,
        T: Clone,
    {
        self.with_ref_ok(|reference| block(reference.clone()))
            .unwrap_or(default)
    }
}

impl<T: Any> ValueBoxPointer<T> for *mut ValueBox<T> {
    fn take_value(&self) -> Result<T> {
        if self.is_null() {
            return BoxerError::NullPointer(type_name::<T>().to_string()).into();
        }

        let value_box = unsafe { *from_raw(*self) };
        Ok(value_box.into_value())
    }

    fn with_ref<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&T) -> Result<R>,
    {
        if self.is_null() {
            return BoxerError::NullPointer(type_name::<T>().to_string()).into();
        }

        unsafe { op(&(**self).value) }
    }

    fn with_mut<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&mut T) -> Result<R>,
    {
        if self.is_null() {
            return BoxerError::NullPointer(type_name::<T>().to_string()).into();
        }

        unsafe { op(&mut (**self).value) }
    }

    fn replace_value<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(T) -> T,
    {
        if self.is_null() {
            return BoxerError::NullPointer(type_name::<T>().to_string()).into();
        }

        let value = unsafe { &mut (**self).value };
        let guard = AbortOnPanic;
        let previous_value = unsafe { std::ptr::read(value) };
        let new_value = op(previous_value);
        unsafe { std::ptr::write(value, new_value) };
        std::mem::forget(guard);
        Ok(())
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn release(self) {
        let result = if self.is_null() {
            BoxerError::NullPointer(type_name::<T>().to_string()).into()
        } else {
            unsafe { Ok(from_raw(self)) }
        };
        result.log();
    }

    fn has_value(&self) -> bool {
        !self.is_null()
    }
}

/// Tell Rust to take back the control over memory
/// This is dangerous! Rust takes control over the memory back
///
/// # Safety
///
/// `pointer` must come from [`Box::into_raw`] for the same `T`, must not be
/// null, and must not have already been reclaimed.
pub unsafe fn from_raw<T>(pointer: *mut T) -> Box<T> {
    assert!(!pointer.is_null(), "from_raw(): Pointer must not be null!");
    assert_eq!(
        size_of::<*mut T>(),
        size_of::<*mut std::ffi::c_void>(),
        "The pointer must be compatible with void*"
    );
    unsafe { Box::from_raw(pointer) }
}

pub fn into_raw<T>(_box: Box<T>) -> *mut T {
    assert_eq!(
        size_of::<*mut T>(),
        size_of::<*mut std::ffi::c_void>(),
        "The pointer must be compatible with void*"
    );
    Box::into_raw(_box)
}

struct AbortOnPanic;

impl Drop for AbortOnPanic {
    fn drop(&mut self) {
        if std::thread::panicking() {
            std::process::abort();
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(deprecated)]
    #![allow(dead_code)]

    use std::error::Error;
    use std::ffi::c_void;
    use std::fmt::{Display, Formatter};
    use std::mem::size_of;
    use std::rc::Rc;

    use crate::value_box::{ValueBox, ValueBoxPointer};

    use super::*;

    #[derive(Debug)]
    pub struct CustomError {}

    impl Display for CustomError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("CustomError")
        }
    }

    impl Error for CustomError {}

    #[test]
    pub fn value_box_size_in_memory() -> Result<()> {
        assert_eq!(size_of::<ValueBox<c_void>>(), size_of::<c_void>());
        assert_eq!(size_of::<ValueBox<(u64, u64)>>(), size_of::<(u64, u64)>());
        assert_eq!(size_of::<ValueBox<()>>(), size_of::<()>());
        assert_eq!(
            size_of::<ValueBox<Box<dyn Error>>>(),
            size_of::<Box<dyn Error>>()
        );

        Ok(())
    }

    #[test]
    pub fn value_box_as_ref_mut() -> Result<()> {
        let value_box = ValueBox::new(5);
        let value_box_ptr = value_box.into_raw();
        let value = value_box_ptr.with_ref_ok(|value| *value)?;
        assert_eq!(value, 5);
        value_box_ptr.release();

        Ok(())
    }

    #[test]
    fn value_box_with_not_null_value() {
        let value_box = ValueBox::new(5);

        let value_box_ptr = value_box.into_raw();
        assert!(!value_box_ptr.is_null());

        let mut result = 0;
        value_box_ptr.with_not_null_value(|value| result = value * 2);
        assert!(!value_box_ptr.is_null());
        assert_eq!(result, 10);

        value_box_ptr.release();
    }

    #[test]
    fn value_box_with_not_null_value_return() {
        let value_box = ValueBox::new(5);

        let value_box_ptr = value_box.into_raw();
        assert!(!value_box_ptr.is_null());

        let result = value_box_ptr.with_not_null_value_return(0, |value| value * 2);
        assert!(!value_box_ptr.is_null());
        assert_eq!(result, 10);

        value_box_ptr.release();
    }

    #[test]
    fn value_box_drop() {
        let value = Rc::new(42);

        let ptr = ValueBox::new(value.clone()).into_raw();
        assert_eq!(Rc::strong_count(&value), 2);
        ptr.release();

        assert_eq!(Rc::strong_count(&value), 1);
    }
}
