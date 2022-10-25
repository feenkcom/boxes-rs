#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct SizeBox<T>
where
    T: From<u8> + Default + Copy,
{
    pub width: T,
    pub height: T,
}

impl<T> SizeBox<T>
where
    T: From<u8> + Default + Copy,
{
    pub fn be_zero(&mut self) {
        self.width = 0u8.into();
        self.height = 0u8.into();
    }

    pub fn new(width: T, height: T) -> Self {
        SizeBox::<T> { width, height }
    }
}
