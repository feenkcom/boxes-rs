use std::ops::Range;

use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn boxer_range_usize_create() -> OwnedPtr<Range<usize>> {
    OwnedPtr::new(0..0)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_range_usize_drop(range: OwnedPtr<Range<usize>>) {
    drop(range);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_range_usize_get_start(range: BorrowedPtr<Range<usize>>) -> usize {
    range.with_ref_ok(|range| range.start).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_range_usize_set_start(mut range: BorrowedPtr<Range<usize>>, start: usize) {
    range.with_mut_ok(|range| range.start = start).log();
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_range_usize_get_end(range: BorrowedPtr<Range<usize>>) -> usize {
    range.with_ref_ok(|range| range.end).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_range_usize_set_end(mut range: BorrowedPtr<Range<usize>>, end: usize) {
    range.with_mut_ok(|range| range.end = end).log();
}
