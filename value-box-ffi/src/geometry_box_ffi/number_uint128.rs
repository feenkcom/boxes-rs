use geometry_box::U128Box;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn boxer_number_uint128_create() -> OwnedPtr<U128Box> {
    OwnedPtr::new(U128Box::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_number_uint128_drop(ptr: OwnedPtr<U128Box>) {
    drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_number_uint128_get_low(number: BorrowedPtr<U128Box>) -> u64 {
    number.with_ref_ok(|number| number.low).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_number_uint128_set_low(mut number: BorrowedPtr<U128Box>, low: u64) {
    number.with_mut_ok(|number| number.low = low).log();
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_number_uint128_get_high(number: BorrowedPtr<U128Box>) -> u64 {
    number.with_ref_ok(|number| number.high).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_number_uint128_set_high(mut number: BorrowedPtr<U128Box>, high: u64) {
    number.with_mut_ok(|number| number.high = high).log();
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_number_uint128_set_max(mut number: BorrowedPtr<U128Box>) {
    number.with_mut_ok(|number| number.set(u128::MAX)).log();
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_number_uint128_set_min(mut number: BorrowedPtr<U128Box>) {
    number.with_mut_ok(|number| number.set(u128::MIN)).log();
}
