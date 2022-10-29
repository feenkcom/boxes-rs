#[derive(Debug)]
#[repr(C)]
pub struct ArrayBox<T> {
    pub data: *mut T,
    pub length: usize,
    pub capacity: usize,
    pub owned: bool,
}

impl<T> ArrayBox<T> {
    pub fn new() -> Self {
        ArrayBox {
            length: 0,
            capacity: 0,
            data: std::ptr::null_mut(),
            owned: true,
        }
    }

    pub fn from_vector(vector: Vec<T>) -> Self {
        let mut array = Self::new();
        array.set_vector(vector);
        array
    }

    /// I create a copy of a given array
    pub fn from_array(array_buffer: &[T]) -> Self
    where
        T: Clone,
    {
        Self::from_vector(Vec::<T>::from(array_buffer))
    }

    /// Create an array assuming that I don't own the data
    pub fn from_data(data: *mut T, length: usize) -> Self {
        ArrayBox {
            length,
            capacity: length,
            data,
            owned: false,
        }
    }

    /// Mutate me to hold a given vector
    pub fn set_vector(&mut self, vector: Vec<T>) {
        // first free existing char buffer
        Self::free_buffer(self.data, self.length, self.capacity, self.owned);
        let mut data = vector;
        data.shrink_to_fit();

        self.length = data.len();
        self.capacity = data.capacity();
        self.data = Self::vec_to_buffer(data)
    }

    /// Mutate me to hold a given vector
    pub fn set_array(&mut self, array_buffer: &[T])
    where
        T: Clone,
    {
        let vector = Vec::<T>::from(array_buffer);
        self.set_vector(vector);
    }

    pub fn to_slice(&self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.length) }
    }

    pub fn copy_into(&self, another_array: &mut ArrayBox<T>) {
        assert!(
            self.length <= another_array.length,
            "The source does not fit into destination"
        );
        assert!(!self.data.is_null(), "The source data must not be nil");
        assert!(
            !another_array.data.is_null(),
            "The destination data must not be nil"
        );
        unsafe { std::ptr::copy_nonoverlapping::<T>(self.data, another_array.data, self.length) }
    }

    pub fn to_vector(mut self) -> Vec<T>
    where
        T: Clone,
    {
        let vector = unsafe { Vec::from_raw_parts(self.data, self.length, self.capacity) };
        if self.owned {
            // I do not own data anymore
            self.owned = false;
            self.data = std::ptr::null_mut();
            vector
        } else {
            let clone = vector.clone();
            // do not de-allocate
            std::mem::forget(vector);
            clone
        }
    }

    pub fn at_put(&mut self, index: usize, object: T) {
        assert!(index < self.length, "Index must be less than array length");

        let slice = self.to_slice();
        slice[index] = object;
    }

    pub fn at(&self, index: usize) -> T
    where
        T: Clone,
    {
        assert!(index < self.length, "Index must be less than array length");

        let slice = self.to_slice();
        slice[index].clone()
    }
}

impl<T> ArrayBox<T> {
    fn vec_to_buffer(mut _data: Vec<T>) -> *mut T {
        let _ptr = _data.as_mut_ptr();
        std::mem::forget(_data);
        _ptr
    }

    fn free_buffer(_ptr_data: *mut T, _length: usize, _capacity: usize, _owned: bool) {
        if _ptr_data.is_null() {
            return;
        }
        if !_owned {
            return;
        }
        drop(unsafe { Vec::from_raw_parts(_ptr_data, _length, _capacity) });
    }
}

impl<T> Default for ArrayBox<T> {
    fn default() -> Self {
        ArrayBox::from_vector(vec![])
    }
}

impl<T> Drop for ArrayBox<T> {
    fn drop(&mut self) {
        Self::free_buffer(self.data, self.length, self.capacity, self.owned);
        self.data = std::ptr::null_mut();
        self.length = 0;
        self.capacity = 0;
    }
}

impl<T> ArrayBox<T>
where
    T: Default + Copy,
{
    pub fn byte_size(count: usize) -> usize {
        std::mem::size_of::<T>() * count
    }

    pub fn new_with(element: T, amount: usize) -> ArrayBox<T> {
        ArrayBox::<T>::from_vector(vec![element; amount])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_array_u8() {
        let array = ArrayBox::<u8>::default();
        assert_eq!(array.capacity, 0);
        assert_eq!(array.length, 0);
        assert_eq!(array.data.is_null(), false);
    }

    #[test]
    fn new_array_u8() {
        let array = ArrayBox::<u8>::from_vector(vec![0, 1, 2, 3, 4]);
        assert_eq!(array.capacity, 5);
        assert_eq!(array.length, 5);
        assert_eq!(array.data.is_null(), false);
    }
}
