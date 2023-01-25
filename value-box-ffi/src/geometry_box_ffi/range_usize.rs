use std::ops::Range;

use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn boxer_range_usize_create() -> *mut ValueBox<Range<usize>> {
    ValueBox::new(0..0).into_raw()
}

#[no_mangle]
pub extern "C" fn boxer_range_usize_drop(range: *mut ValueBox<Range<usize>>) {
    range.release();
}

#[no_mangle]
pub extern "C" fn boxer_range_usize_get_start(range: *mut ValueBox<Range<usize>>) -> usize {
    range.with_ref_ok(|range| range.start).or_log(0)
}

#[no_mangle]
pub extern "C" fn boxer_range_usize_set_start(range: *mut ValueBox<Range<usize>>, start: usize) {
    range.with_mut_ok(|range| range.start = start).log();
}

#[no_mangle]
pub extern "C" fn boxer_range_usize_get_end(range: *mut ValueBox<Range<usize>>) -> usize {
    range.with_ref_ok(|range| range.end).or_log(0)
}

#[no_mangle]
pub extern "C" fn boxer_range_usize_set_end(range: *mut ValueBox<Range<usize>>, end: usize) {
    range.with_mut_ok(|range| range.end = end).log();
}
