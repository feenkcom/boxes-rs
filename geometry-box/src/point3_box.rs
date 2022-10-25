#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Point3Box<T>
where
    T: From<u8> + Default + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3Box<T>
where
    T: From<u8> + Default + Copy,
{
    pub fn be_zero(&mut self) {
        self.x = 0u8.into();
        self.y = 0u8.into();
        self.z = 0u8.into();
    }

    pub fn new(x: T, y: T, z: T) -> Self {
        Point3Box::<T> { x, y, z }
    }
}
