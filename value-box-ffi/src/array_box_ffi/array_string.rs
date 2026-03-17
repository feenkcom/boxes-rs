use array_box::ArrayBox;
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_create() -> OwnedPtr<ArrayBox<StringBox>> {
    OwnedPtr::new(ArrayBox::new())
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_get_length(array: BorrowedPtr<ArrayBox<StringBox>>) -> usize {
    array.with_ref_ok(|array| array.length).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_at(
    array: BorrowedPtr<ArrayBox<StringBox>>,
    index: usize,
    _item: BorrowedPtr<StringBox>,
) -> OwnedPtr<StringBox> {
    array
        .with_ref_ok(|array| OwnedPtr::new(array.at(index)))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_at_put(
    mut array: BorrowedPtr<ArrayBox<StringBox>>,
    index: usize,
    item: OwnedPtr<StringBox>,
) {
    array
        .with_mut(|array| item.take_value().map(|item| array.at_put(index, item)))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_string_drop(array: OwnedPtr<ArrayBox<StringBox>>) {
    drop(array);
}
