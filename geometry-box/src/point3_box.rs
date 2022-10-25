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

    // pub fn boxer_point_default() -> *mut ValueBox<Point3Box<T>> {
    //     ValueBox::new(Point3Box::<T>::default()).into_raw()
    // }
    //
    // pub fn boxer_point_create(x: T, y: T, z: T) -> *mut ValueBox<Point3Box<T>> {
    //     ValueBox::new(Point3Box::<T>::new(x, y, z)).into_raw()
    // }
    //
    // pub fn boxer_point_drop(ptr: *mut ValueBox<Point3Box<T>>) {
    //     ptr.release();
    // }
    //
    // pub fn boxer_point_get_x(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T {
    //     _maybe_null_ptr.with_not_null_return(0u8.into(), |point| point.x)
    // }
    //
    // pub fn boxer_point_set_x(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, x: T) {
    //     _maybe_null_ptr.with_not_null(|point| point.x = x)
    // }
    //
    // pub fn boxer_point_get_y(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T {
    //     _maybe_null_ptr.with_not_null_return(0u8.into(), |point| point.y)
    // }
    //
    // pub fn boxer_point_set_y(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, y: T) {
    //     _maybe_null_ptr.with_not_null(|point| point.y = y)
    // }
    //
    // pub fn boxer_point_get_z(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>) -> T {
    //     _maybe_null_ptr.with_not_null_return(0u8.into(), |point| point.z)
    // }
    //
    // pub fn boxer_point_set_z(_maybe_null_ptr: *mut ValueBox<Point3Box<T>>, z: T) {
    //     _maybe_null_ptr.with_not_null(|point| point.z = z)
    // }
}
