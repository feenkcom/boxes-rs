use array_box::ArrayBox;
use string_box::StringBox;
use value_box::{value_box, ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_create() -> *mut ValueBox<ArrayBox<StringBox>> {
    value_box!(ArrayBox::new()).into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_get_length(
    array: *mut ValueBox<ArrayBox<StringBox>>,
) -> usize {
    array.with_ref_ok(|array| array.length).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_at(
    array: *mut ValueBox<ArrayBox<StringBox>>,
    index: usize,
    _item: *mut ValueBox<StringBox>,
) -> *mut ValueBox<StringBox> {
    array
        .with_ref_ok(|array| value_box!(array.at(index)))
        .into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_at_put(
    array: *mut ValueBox<ArrayBox<StringBox>>,
    index: usize,
    item: *mut ValueBox<StringBox>,
) {
    array
        .with_mut(|array| item.take_value().map(|item| array.at_put(index, item)))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_drop(array: *mut ValueBox<ArrayBox<StringBox>>) {
    array.release();
}
