use std::any::{type_name, Any};
use std::fmt::{Debug, Formatter};
use std::mem::ManuallyDrop;

use crate::{BoxerError, Result, ReturnBoxerResult, ValueBoxContainer};

#[repr(C, u8)]
pub enum ValueBox<T: Any> {
    Value(Option<Box<T>>),
    #[cfg(feature = "phlow")]
    PhlowValue(Box<crate::PhlowValue>),
}

impl<T: Any> ValueBox<T> {
    pub fn new(object: T) -> Self {
        Self::Value(Some(Box::new(object)))
    }

    #[cfg(feature = "phlow")]
    pub fn new_phlow(object: T, phlow_type_fn: fn() -> phlow::PhlowType) -> Self {
        Self::PhlowValue(Box::new(crate::PhlowValue::new(object, phlow_type_fn)))
    }

    pub fn null() -> Self {
        Self::Value(None)
    }

    pub fn has_value(&self) -> bool {
        match self {
            Self::Value(value) => value.has_value(),
            #[cfg(feature = "phlow")]
            Self::PhlowValue(value) => {
                <crate::PhlowValue as ValueBoxContainer<T>>::has_value(value)
            }
        }
    }

    pub fn replace_value(&mut self, object: T) -> Option<T> {
        match self {
            Self::Value(value) => value.replace_value(object),
            #[cfg(feature = "phlow")]
            Self::PhlowValue(value) => value.replace_value(object),
        }
    }

    pub fn set_value(&mut self, object: T) {
        self.replace_value(object);
    }

    pub fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Self::Value(value) => value.clone_value(),
            #[cfg(feature = "phlow")]
            Self::PhlowValue(value) => value.clone_value(),
        }
    }

    pub fn take_value(&mut self) -> Option<T> {
        match self {
            Self::Value(value) => value.take_value(),
            #[cfg(feature = "phlow")]
            Self::PhlowValue(value) => value.take_value(),
        }
    }

    pub fn into_raw(self) -> *mut Self {
        into_raw(Box::new(self))
    }
}

impl<T: 'static> ValueBox<T> {
    #[cfg(feature = "phlow")]
    /// Try to get a phlow object from the lazily defined phlow value.
    /// Note: By definition `PhlowObject` owns the value, therefore
    /// internally we change the storage container for the value to `PhlowObject`.
    pub fn phlow_object(&mut self) -> Option<phlow::PhlowObject> {
        match self {
            Self::Value(_) => None,
            Self::PhlowValue(value) => value.phlow_object(),
        }
    }
}

impl<T: Any> Drop for ValueBox<T> {
    fn drop(&mut self) {
        trace!(
            "Dropping {} of {}",
            (if self.has_value() { "Some" } else { "None" }),
            type_name::<T>()
        );
    }
}

#[repr(transparent)]
pub struct BoxRef<T: Any> {
    value_box: ManuallyDrop<Box<ValueBox<T>>>,
}

impl<T: Any> BoxRef<T> {
    pub fn with_ref<R>(&self, op: impl FnOnce(&T) -> Result<R>) -> Result<R> {
        match self.value_box.as_ref() {
            ValueBox::Value(value) => op(value.as_ref().unwrap()),
            #[cfg(feature = "phlow")]
            ValueBox::PhlowValue(value) => match value.as_ref() {
                crate::PhlowValue::Lazy(value) => {
                    let value = &value.value;
                    op(value.as_ref_safe::<T>().unwrap())
                }
                crate::PhlowValue::Object(object) => {
                    let value = object.value_ref::<T>().unwrap();
                    op(&value)
                }
            },
        }
    }

    pub fn with_mut<R>(&mut self, op: impl FnOnce(&mut T) -> Result<R>) -> Result<R> {
        match self.value_box.as_mut() {
            ValueBox::Value(value) => value
                .as_mut()
                .ok_or_else(|| BoxerError::NoValue(type_name::<T>().to_string()))
                .and_then(|value| op(value)),
            #[cfg(feature = "phlow")]
            ValueBox::PhlowValue(value) => match value.as_mut() {
                crate::PhlowValue::Lazy(value) => value
                    .value
                    .as_mut_safe::<T>()
                    .ok_or_else(|| BoxerError::NoValue(type_name::<T>().to_string()))
                    .and_then(|value| op(value)),
                crate::PhlowValue::Object(object) => {
                    let mut value = object.value_mut::<T>().unwrap();
                    op(&mut value)
                }
            },
        }
    }
}

impl<T: Any> Debug for BoxRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxRef")
            .field(
                "value",
                if self.value_box.has_value() {
                    &"Some"
                } else {
                    &"None"
                },
            )
            .finish()
    }
}

impl<T: Any> BoxRef<T> {
    pub fn replace(&mut self, value: T) -> Option<T> {
        self.value_box.replace_value(value)
    }

    pub fn take_value(&mut self) -> Option<T> {
        self.value_box.take_value()
    }
}

pub trait ValueBoxPointer<T: Any> {
    /// Get the reference to the underlying box without dropping it.
    fn to_ref(&self) -> Result<BoxRef<T>>;

    /// Take the value out of the box.
    fn take_value(&self) -> Result<T>;

    /// Evaluate a given function with a reference to the boxed value.
    /// The the reference can not outlive the closure.
    fn with_ref<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&T) -> Result<R>,
    {
        self.to_ref()?.with_ref(op)
    }

    /// Try to unbox the value and evaluate a given function with either Some
    /// if the value was there or None if the pointer was a null pointer.
    /// Returns an error if the value box wasn't a null pointer, but the boxed value
    /// was already taken or of the wrong type.
    fn with_option_ref<R: Any, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(Option<&T>) -> Result<R>,
    {
        match self.to_ref() {
            Ok(value) => value.with_ref(|value| op(Some(value))),
            Err(_) => op(None),
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
        F: FnOnce(&mut T) -> Result<R>,
    {
        self.to_ref()?.with_mut(op)
    }

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
        self.to_ref()?.with_ref(|value| op(value.clone()))
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
            ptr1.with_ref(|p1| ptr2.with_ref(|p2| ptr3.with_ref(|p3| op(&t, &p1, &p2, &p3))))
        })
    }

    /// Evaluate a given function with the value taken out of the box
    /// and place the new value back. The value returned by the function
    /// must be of the same type as the box
    fn replace_value<F>(&self, op: F) -> Result<()>
    where
        F: FnOnce(T) -> T,
    {
        self.to_ref().and_then(|mut t| {
            t.take_value()
                .ok_or_else(|| BoxerError::NoValue(type_name::<T>().to_string()))
                .map(|previous_value| {
                    let new_value = op(previous_value);
                    t.replace(new_value);
                })
        })
    }

    fn release(self);

    fn has_value(&self) -> bool {
        self.to_ref().map(|_| true).unwrap_or(false)
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
        self.with_clone_ok(|value| block(value))
            .unwrap_or_else(|_| default())
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
    fn to_ref(&self) -> Result<BoxRef<T>> {
        if self.is_null() {
            return BoxerError::NullPointer(type_name::<T>().to_string()).into();
        }
        let value_box = ManuallyDrop::new(unsafe { from_raw(*self) });

        if value_box.has_value() {
            Ok(BoxRef { value_box })
        } else {
            BoxerError::NoValue(type_name::<T>().to_string()).into()
        }
    }

    fn take_value(&self) -> Result<T> {
        if self.is_null() {
            return BoxerError::NullPointer(type_name::<T>().to_string()).into();
        }
        let mut value_box = ManuallyDrop::new(unsafe { from_raw(*self) });
        value_box
            .take_value()
            .ok_or(BoxerError::NoValue(type_name::<T>().to_string()))
    }

    fn release(self) {
        let result = if self.is_null() {
            BoxerError::NullPointer(type_name::<T>().to_string()).into()
        } else {
            unsafe { Ok(from_raw(self)) }
        };
        result.log();
    }
}

/// Tell Rust to take back the control over memory
/// This is dangerous! Rust takes the control over the memory back
pub unsafe fn from_raw<T>(pointer: *mut T) -> Box<T> {
    assert_eq!(
        pointer.is_null(),
        false,
        "from_raw(): Pointer must not be null!"
    );
    assert_eq!(
        std::mem::size_of::<*mut T>(),
        std::mem::size_of::<*mut std::ffi::c_void>(),
        "The pointer must be compatible with void*"
    );
    Box::from_raw(pointer)
}

pub fn into_raw<T>(_box: Box<T>) -> *mut T {
    assert_eq!(
        std::mem::size_of::<*mut T>(),
        std::mem::size_of::<*mut std::ffi::c_void>(),
        "The pointer must be compatible with void*"
    );
    Box::into_raw(_box)
}

#[cfg(test)]
mod test {
    #![allow(deprecated)]
    #![allow(dead_code)]

    use std::error::Error;
    use std::fmt::Display;
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
    pub fn value_box_as_ref_mut() -> Result<()> {
        let value_box = ValueBox::new(5);
        let value_box_ptr = value_box.into_raw();
        let value = value_box_ptr.with_ref_ok(|value| value.clone())?;
        assert_eq!(value, 5);

        Ok(())
    }

    #[test]
    fn value_box_with_not_null_value() {
        let value_box = ValueBox::new(5);

        let value_box_ptr = value_box.into_raw();
        assert_eq!(value_box_ptr.is_null(), false);

        let mut result = 0;
        value_box_ptr.with_not_null_value(|value| result = value * 2);
        assert_eq!(value_box_ptr.is_null(), false);
        assert_eq!(result, 10);

        value_box_ptr.release();
    }

    #[test]
    fn value_box_with_not_null_value_return() {
        let value_box = ValueBox::new(5);

        let value_box_ptr = value_box.into_raw();
        assert_eq!(value_box_ptr.is_null(), false);

        let result = value_box_ptr.with_not_null_value_return(0, |value| value * 2);
        assert_eq!(value_box_ptr.is_null(), false);
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
