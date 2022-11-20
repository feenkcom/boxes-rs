use std::any::type_name;
use std::fmt::{Debug, Formatter};
use std::intrinsics::transmute;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

use crate::{BoxerError, Result, ReturnBoxerResult};

#[repr(transparent)]
pub struct ValueBox<T> {
    value: Option<T>,
}

impl<T> ValueBox<T> {
    pub fn new(object: T) -> Self {
        ValueBox {
            value: Some(object),
        }
    }

    pub fn null() -> Self {
        ValueBox { value: None }
    }

    pub fn has_value(&self) -> bool {
        trace!("[has_value] value pointer: {:?}", unsafe {
            transmute::<Option<&T>, *const T>(self.value.as_ref())
        });
        self.value.is_some()
    }

    pub fn replace(&mut self, value: T) -> Option<T> {
        self.value.replace(value)
    }

    pub fn set_value(&mut self, object: T) {
        self.value = Some(object)
    }

    pub fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        self.value.clone()
    }

    pub fn take_value(&mut self) -> Option<T> {
        self.value.take()
    }

    pub fn into_raw(self) -> *mut Self {
        into_raw(Box::new(self))
    }

    pub fn as_ref_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    pub fn as_ptr(&self) -> *const T {
        self.value
            .as_ref()
            .map_or(std::ptr::null(), |reference| reference as *const T)
    }

    pub fn as_ptr_mut(&mut self) -> *mut T {
        self.value
            .as_mut()
            .map_or(std::ptr::null_mut(), |reference| reference as *mut T)
    }
}

impl<T> Drop for ValueBox<T> {
    fn drop(&mut self) {
        debug!(
            "Dropping {} of {}",
            self.value.as_ref().map_or("None", |_| { "Some" }),
            type_name::<T>()
        );
    }
}

#[repr(transparent)]
pub struct BoxRef<T> {
    value_box: ManuallyDrop<Box<ValueBox<T>>>,
}

impl<T> Deref for BoxRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value_box.deref().value.as_ref().unwrap()
    }
}

impl<T> DerefMut for BoxRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value_box.deref_mut().value.as_mut().unwrap()
    }
}

impl<T> Debug for BoxRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxRef")
            .field(
                "value",
                self.value_box
                    .deref()
                    .value
                    .as_ref()
                    .map_or(&"None", |_| &"Some"),
            )
            .finish()
    }
}

impl<T> BoxRef<T> {
    pub fn replace(&mut self, value: T) -> Option<T> {
        self.value_box.replace(value)
    }

    pub fn as_ptr(&self) -> *const T {
        self.value_box.as_ptr()
    }

    pub fn take_value(&mut self) -> Option<T> {
        self.value_box.take_value()
    }
}

pub trait ValueBoxPointer<T> {
    /// Get the reference to the underlying value without dropping it.
    fn to_ref(&self) -> Result<BoxRef<T>>;

    /// Take the value out of the box.
    fn take_value(&self) -> Result<T>;

    /// Evaluate a given function with a reference to the boxed value.
    /// The lifetime of the reference can not outlive the closure.
    fn with_ref<R, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&T) -> R,
    {
        self.to_ref().map(|t| op(&t))
    }

    /// Evaluate a given function with references to given boxed values.
    /// The lifetime of the reference can not outlive the closure.
    fn with_ref_ref<R, F, P>(&self, ptr: *mut ValueBox<P>, op: F) -> Result<R>
    where
        F: FnOnce(&T, &P) -> R,
    {
        self.to_ref().and_then(|t| ptr.to_ref().map(|p| op(&t, &p)))
    }

    /// Evaluate a given function with references to given boxed values.
    /// The lifetime of the reference can not outlive the closure.
    fn with_ref_ref_ref<R, F, P1, P2>(
        &self,
        ptr1: *mut ValueBox<P1>,
        ptr2: *mut ValueBox<P2>,
        op: F,
    ) -> Result<R>
    where
        F: FnOnce(&T, &P1, &P2) -> R,
    {
        self.to_ref().and_then(|t| {
            ptr1.to_ref()
                .and_then(|p1| ptr2.to_ref().map(|p2| op(&t, &p1, &p2)))
        })
    }

    /// Evaluate a given function with references to given boxed values.
    /// The lifetime of the reference can not outlive the closure.
    fn with_ref_ref_ref_ref<R, F, P1, P2, P3>(
        &self,
        ptr1: *mut ValueBox<P1>,
        ptr2: *mut ValueBox<P2>,
        ptr3: *mut ValueBox<P3>,
        op: F,
    ) -> Result<R>
    where
        F: FnOnce(&T, &P1, &P2, &P3) -> R,
    {
        self.to_ref().and_then(|t| {
            ptr1.to_ref().and_then(|p1| {
                ptr2.to_ref()
                    .and_then(|p2| ptr3.to_ref().map(|p3| op(&t, &p1, &p2, &p3)))
            })
        })
    }

    /// Evaluate a given function with a mutable reference to the boxed value.
    /// The lifetime of the reference can not outlive the closure.
    fn with_mut<R, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.to_ref().map(|mut t| op(&mut t))
    }

    /// Evaluate a given function with a clone of the boxed value.
    /// The boxed type `T` must implement [`Clone`].
    fn with_clone<R, F>(&self, op: F) -> Result<R>
    where
        F: FnOnce(T) -> R,
        T: Clone,
    {
        self.to_ref().map(|t| op(t.clone()))
    }

    fn release(self);

    fn has_value(&self) -> bool {
        self.to_ref().map(|_| true).unwrap_or(false)
    }

    fn get_ptr(&self) -> *const T {
        self.to_ref()
            .map(|reference| reference.as_ptr())
            .or_log(std::ptr::null())
    }

    #[deprecated(since = "0.1.0", note = "please use `take_value` instead")]
    fn to_value(&self) -> Result<T> {
        self.take_value()
    }

    #[deprecated(since = "0.1.0", note = "please use `has_value` instead")]
    fn is_valid(&self) -> bool {
        self.has_value()
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref()?.replace()` instead")]
    fn mutate(&self, object: T) {
        self.to_ref()
            .and_then(|mut reference| Ok(reference.replace(object)))
            .log();
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with_box<DefaultBlock, Block, Return>(&self, default: DefaultBlock, block: Block) -> Return
    where
        DefaultBlock: FnOnce() -> Return,
        Block: FnOnce(&mut Box<ValueBox<T>>, DefaultBlock) -> Return,
    {
        if let Ok(mut reference) = self.to_ref() {
            block(reference.value_box.deref_mut(), default)
        } else {
            default()
        }
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with<DefaultBlock, Block, Return>(&self, default: DefaultBlock, block: Block) -> Return
    where
        DefaultBlock: FnOnce() -> Return,
        Block: FnOnce(&mut T) -> Return,
    {
        self.to_ref()
            .and_then(|mut reference| Ok(block(reference.deref_mut())))
            .unwrap_or_else(|_| default())
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with_not_null<Block>(&self, block: Block)
    where
        Block: FnOnce(&mut T),
    {
        self.to_ref()
            .and_then(|mut reference| Ok(block(reference.deref_mut())))
            .log();
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with_not_null_return<Block, Return>(&self, default: Return, block: Block) -> Return
    where
        Block: FnOnce(&mut T) -> Return,
    {
        self.to_ref()
            .map(|mut reference| block(reference.deref_mut()))
            .or_log(default)
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with_value<DefaultBlock, Block, Return>(&self, default: DefaultBlock, block: Block) -> Return
    where
        DefaultBlock: FnOnce() -> Return,
        Block: FnOnce(T) -> Return,
        T: Clone,
    {
        self.to_ref()
            .and_then(|reference| Ok(block(reference.clone())))
            .unwrap_or_else(|_| default())
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with_not_null_value<Block>(&self, block: Block)
    where
        Block: FnOnce(T),
        T: Clone,
    {
        self.to_ref()
            .and_then(|reference| Ok(block(reference.clone())))
            .log();
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with_not_null_value_return<Block, Return>(&self, default: Return, block: Block) -> Return
    where
        Block: FnOnce(T) -> Return,
        T: Clone,
    {
        self.to_ref()
            .and_then(|reference| Ok(block(reference.clone())))
            .unwrap_or(default)
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with_not_null_value_mutate<Block>(&mut self, block: Block)
    where
        Block: FnOnce(T) -> T,
    {
        self.to_ref()
            .map(|mut reference| {
                reference
                    .take_value()
                    .map(block)
                    .and_then(|value| reference.replace(value))
            })
            .log();
    }

    #[deprecated(since = "0.1.0", note = "please use `to_ref` instead")]
    fn with_value_consumed<DefaultBlock, Block, Return>(
        &mut self,
        default: DefaultBlock,
        block: Block,
    ) -> Return
    where
        DefaultBlock: FnOnce() -> Return,
        Block: FnOnce(T, &mut Box<ValueBox<T>>) -> Return,
    {
        self.to_ref()
            .and_then(|mut reference| {
                reference
                    .take_value()
                    .map(|value| block(value, reference.value_box.deref_mut()))
                    .ok_or_else(|| BoxerError::NoValue(type_name::<T>().to_string()).into())
            })
            .unwrap_or_else(|_| default())
    }

    #[deprecated(since = "0.1.0", note = "please use `take_value` instead")]
    fn with_not_null_value_consumed<Block>(&mut self, block: Block)
    where
        Block: FnOnce(T),
    {
        self.take_value().and_then(|value| Ok(block(value))).log();
    }

    #[deprecated(since = "0.1.0", note = "please use `take_value` instead")]
    fn with_not_null_value_consumed_return<Block, Return>(
        &mut self,
        default: Return,
        block: Block,
    ) -> Return
    where
        Block: FnOnce(T) -> Return,
    {
        self.take_value().map(|value| block(value)).or_log(default)
    }
}

impl<T> ValueBoxPointer<T> for *mut ValueBox<T> {
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

    use anyhow::anyhow;

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
    pub fn value_box_as_ref() -> Result<()> {
        let value_box = ValueBox::new(5);
        let value_box_ptr = value_box.into_raw();

        let reference = value_box_ptr.to_ref()?;

        assert_eq!(reference.deref(), &5);
        drop(reference);

        let value = value_box_ptr
            .to_ref()
            .and_then(|_value| Err(anyhow!("New error").into()))
            .unwrap_or(0);
        assert_eq!(value, 0);

        let value = value_box_ptr
            .to_ref()
            .and_then(|_value| Err((Box::new(CustomError {}) as Box<dyn Error>).into()))
            .unwrap_or(0);
        assert_eq!(value, 0);

        Ok(())
    }

    #[test]
    pub fn value_box_as_ref_mut() -> Result<()> {
        let value_box = ValueBox::new(5);
        let value_box_ptr = value_box.into_raw();
        let reference = value_box_ptr.to_ref()?.deref_mut().clone();
        assert_eq!(reference, 5);

        Ok(())
    }

    #[test]
    fn value_box_with_consumed() {
        let value_box = ValueBox::new(5);

        let mut value_box_ptr = value_box.into_raw();
        assert_eq!(value_box_ptr.is_null(), false);
        assert_eq!(value_box_ptr.has_value(), true);

        let result = value_box_ptr.with_not_null_value_consumed_return(0, |value| value * 2);

        assert_eq!(result, 10);
        assert_eq!(value_box_ptr.is_null(), false);
        assert_eq!(value_box_ptr.has_value(), false);
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

    struct Child<'counter> {
        value: i32,
        counter: &'counter mut i32,
    }

    struct Parent<'counter> {
        child: Child<'counter>,
        counter: &'counter mut i32,
    }

    impl<'counter> Drop for Parent<'counter> {
        fn drop(&mut self) {
            println!("destroyed Parent");
            *self.counter += 1;
        }
    }

    impl<'counter> Drop for Child<'counter> {
        fn drop(&mut self) {
            println!("destroyed Child");
            *self.counter += 1;
        }
    }

    fn create_parent<'counter>(
        parents_drop: &'counter mut i32,
        children_drop: &'counter mut i32,
    ) -> Parent<'counter> {
        Parent {
            child: Child {
                value: 5,
                counter: children_drop,
            },
            counter: parents_drop,
        }
    }

    #[test]
    fn drop_parent() {
        let mut parents_drop = 0;
        let mut children_drop = 0;

        let parent = create_parent(&mut parents_drop, &mut children_drop);

        drop(parent);

        assert_eq!(parents_drop, 1);
        assert_eq!(children_drop, 1);
    }

    fn put_parent_in_value_box_without_return(parent: Parent) {
        put_parent_in_value_box_with_return(parent);
    }

    fn put_parent_in_value_box_with_return(parent: Parent) -> *mut ValueBox<Parent> {
        ValueBox::new(parent).into_raw()
    }

    #[test]
    fn leak_parent_by_putting_in_value_box_without_drop() {
        let mut parents_drop = 0;
        let mut children_drop = 0;

        let parent = create_parent(&mut parents_drop, &mut children_drop);

        put_parent_in_value_box_without_return(parent);

        assert_eq!(parents_drop, 0);
        assert_eq!(children_drop, 0);
    }

    #[test]
    fn drop_parent_by_dropping_value_box() {
        let mut parents_drop = 0;
        let mut children_drop = 0;

        let parent = create_parent(&mut parents_drop, &mut children_drop);

        let parent_ptr = put_parent_in_value_box_with_return(parent);
        parent_ptr.release();

        assert_eq!(parents_drop, 1);
        assert_eq!(children_drop, 1);
    }
}
