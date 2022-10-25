#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct PointBox<T>
where
    T: From<u8> + Default + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> PointBox<T>
where
    T: From<u8> + Default + Copy,
{
    pub fn be_zero(&mut self) {
        self.x = 0u8.into();
        self.y = 0u8.into();
    }

    pub fn new(x: T, y: T) -> Self {
        PointBox::<T> { x, y }
    }
}
