use std::cell::Cell;
use std::rc::Rc;

use value_box::{BorrowedPtr, OwnedPtr};

trait PhlowView {
    fn get_title(&self) -> &str;
}

struct TestView;

impl PhlowView for TestView {
    fn get_title(&self) -> &str {
        "Hello"
    }
}

#[derive(Default)]
struct ViewTitle {
    value: String,
}

struct AnySendObject {
    dropped: Rc<Cell<bool>>,
}

impl ViewTitle {
    fn set_string(&mut self, value: String) {
        self.value = value;
    }
}

impl Drop for AnySendObject {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

#[value_box_derive::ffi]
pub fn phlow_view_get_title(phlow_view: &Box<dyn PhlowView>, view_title: &mut ViewTitle) {
    view_title.set_string(phlow_view.get_title().to_string())
}

#[value_box_derive::ffi]
pub fn phlow_view_drop(phlow_view: Box<dyn PhlowView>) {
    drop(phlow_view);
}

#[value_box_derive::ffi]
pub fn phlow_any_send_object_drop(object: AnySendObject) {
    drop(object);
}

#[test]
fn ffi_macro_rewrites_borrowed_arguments() {
    let raw_view = Box::into_raw(Box::new(Box::new(TestView) as Box<dyn PhlowView>));
    let raw_title = Box::into_raw(Box::new(ViewTitle::default()));

    let view = unsafe { BorrowedPtr::from_raw(raw_view) };
    let title = unsafe { BorrowedPtr::from_raw(raw_title) };

    phlow_view_get_title(view, title);

    assert_eq!(unsafe { &*raw_title }.value, "Hello");

    unsafe {
        drop(Box::from_raw(raw_view));
        drop(Box::from_raw(raw_title));
    }
}

struct DroppingView {
    dropped: Rc<Cell<bool>>,
}

impl PhlowView for DroppingView {
    fn get_title(&self) -> &str {
        "Dropped"
    }
}

impl Drop for DroppingView {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

#[test]
fn ffi_macro_rewrites_owned_box_arguments() {
    let dropped = Rc::new(Cell::new(false));
    let view = Box::new(DroppingView {
        dropped: dropped.clone(),
    }) as Box<dyn PhlowView>;

    let owned = OwnedPtr::new(view);
    phlow_view_drop(owned);

    assert!(dropped.get());
}

#[test]
fn ffi_macro_rewrites_owned_plain_arguments() {
    let dropped = Rc::new(Cell::new(false));
    let object = AnySendObject {
        dropped: dropped.clone(),
    };

    let owned = OwnedPtr::new(object);
    phlow_any_send_object_drop(owned);

    assert!(dropped.get());
}
