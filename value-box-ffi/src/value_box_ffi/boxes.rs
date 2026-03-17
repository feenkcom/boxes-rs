use std::os::raw::c_void;
use value_box::BorrowedPtr;

#[unsafe(no_mangle)]
pub extern "C" fn boxer_value_box_is_valid(ptr: BorrowedPtr<c_void>) -> bool {
    !ptr.is_null()
}

#[test]
pub fn test_is_valid() {
    let raw = Box::into_raw(Box::new(42_i32));
    let void_ptr = unsafe { BorrowedPtr::<c_void>::from_raw(raw.cast()) };
    assert!(boxer_value_box_is_valid(void_ptr));
    unsafe { drop(Box::from_raw(raw)) };
}
