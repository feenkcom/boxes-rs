use geometry_box::U128Box;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn boxer_number_uint128_create() -> *mut ValueBox<U128Box> {
    ValueBox::new(U128Box::default()).into_raw()
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_drop(ptr: *mut ValueBox<U128Box>) {
    ptr.release();
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_get_low(number: *mut ValueBox<U128Box>) -> u64 {
    number.with_ref_ok(|number| number.low).or_log(0)
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_set_low(number: *mut ValueBox<U128Box>, low: u64) {
    number.with_mut_ok(|number| number.low = low).log();
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_get_high(number: *mut ValueBox<U128Box>) -> u64 {
    number.with_ref_ok(|number| number.high).or_log(0)
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_set_high(number: *mut ValueBox<U128Box>, high: u64) {
    number.with_mut_ok(|number| number.high = high).log();
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_set_max(number: *mut ValueBox<U128Box>) {
    number.with_mut_ok(|number| number.set(u128::MAX)).log();
}

#[no_mangle]
pub extern "C" fn boxer_number_uint128_set_min(number: *mut ValueBox<U128Box>) {
    number.with_mut_ok(|number| number.set(u128::MIN)).log();
}
