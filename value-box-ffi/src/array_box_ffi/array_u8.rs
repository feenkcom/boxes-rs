use array_box::ArrayBox;
use value_box::{BorrowedPtr, ReturnBoxerResult};

/// In-place convert between color formats
pub fn boxer_array_u8_convert_color_format<Block>(slice: &mut [u8], _converter: Block)
where
    Block: Fn(u32) -> u32 + Send + Copy,
{
    if slice.len().is_multiple_of(4) {
        let slice_u32 = unsafe {
            std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut u32, slice.len() / 4)
        };

        if slice_u32.len() > 512 {
            let threads = 16;
            let chunk_size =
                slice_u32.len() / threads + if slice_u32.len() % threads != 0 { 1 } else { 0 };

            // Scoped threads allow the compiler to prove that no threads will outlive
            // table (which would be bad).
            let _ = crossbeam::scope(|scope| {
                // Chop `table` into disjoint sub-slices.
                for each_chunk in slice_u32.chunks_mut(chunk_size) {
                    // Spawn a thread operating on that subslice.
                    scope.spawn(move |_| {
                        for color in each_chunk {
                            *color = _converter(*color);
                        }
                    });
                }
                // `crossbeam::scope` ensures that *all* spawned threads join before
                // returning control back from this closure.
            });
        } else {
            for color in slice_u32 {
                *color = _converter(*color);
            }
        }
    }
}

#[inline]
fn argb_to_rgba(argb: u32) -> u32 {
    argb.rotate_right(8)
}

#[inline]
fn bgra_to_argb(bgra: u32) -> u32 {
    bgra.swap_bytes()
}

#[inline]
fn rgba_to_argb(rgba: u32) -> u32 {
    rgba.rotate_left(8)
}

/// In-place convert argb to rgba
#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_u8_argb_to_rgba(mut array: BorrowedPtr<ArrayBox<u8>>) {
    array
        .with_mut_ok(|array| {
            boxer_array_u8_convert_color_format(array.to_slice_mut(), argb_to_rgba)
        })
        .log();
}

/// In-place convert bgra to argb
#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_u8_bgra_to_argb(mut array: BorrowedPtr<ArrayBox<u8>>) {
    array
        .with_mut_ok(|array| {
            boxer_array_u8_convert_color_format(array.to_slice_mut(), bgra_to_argb);
        })
        .log();
}

/// In-place convert rgba to argb
#[unsafe(no_mangle)]
pub extern "C" fn boxer_array_u8_rgba_to_argb(mut array: BorrowedPtr<ArrayBox<u8>>) {
    array
        .with_mut_ok(|array| {
            boxer_array_u8_convert_color_format(array.to_slice_mut(), rgba_to_argb);
        })
        .log();
}

#[cfg(test)]
mod tests {
    use array_box::ArrayBox;

    use super::*;
    use crate::*;

    fn borrowed_array(
        values: Vec<u8>,
    ) -> (*mut ArrayBox<u8>, value_box::BorrowedPtr<ArrayBox<u8>>) {
        let raw = Box::into_raw(Box::new(ArrayBox::from_vector(values)));
        let borrowed = unsafe { value_box::BorrowedPtr::from_raw(raw) };
        (raw, borrowed)
    }

    #[test]
    fn test_argb_to_rgba() {
        let (raw, argb) = borrowed_array(vec![255, 0, 100, 200]);

        boxer_array_u8_argb_to_rgba(argb);

        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 0),
            0
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 1),
            100
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 2),
            200
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 3),
            255
        );
        unsafe { drop(Box::from_raw(raw)) };
    }

    #[test]
    fn test_rgba_to_argb() {
        let (raw, rgba) = borrowed_array(vec![0, 100, 200, 255]);

        boxer_array_u8_rgba_to_argb(rgba);

        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 0),
            255
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 1),
            0
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 2),
            100
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 3),
            200
        );
        unsafe { drop(Box::from_raw(raw)) };
    }

    #[test]
    fn test_bgra_to_argb() {
        let (raw, bgra) = borrowed_array(vec![0, 100, 200, 255]);

        boxer_array_u8_bgra_to_argb(bgra);

        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 0),
            255
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 1),
            200
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 2),
            100
        );
        assert_eq!(
            boxer_array_u8_at(unsafe { value_box::BorrowedPtr::from_raw(raw) }, 3),
            0
        );
        unsafe { drop(Box::from_raw(raw)) };
    }
}
