#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct U128Box {
    pub high: u64,
    pub low: u64,
}

impl U128Box {
    pub fn from(number: u128) -> Self {
        number.into()
    }

    pub fn be_zero(&mut self) {
        self.set(0);
    }

    pub fn set(&mut self, number: u128) {
        let boxed_number: U128Box = number.into();
        self.clone_from(&boxed_number);
    }

    pub fn get(&self) -> u128 {
        self.into()
    }
}

impl From<&U128Box> for u128 {
    fn from(boxed_number: &U128Box) -> u128 {
        ((boxed_number.high as u128) << 64) + (boxed_number.low as u128)
    }
}

impl From<u128> for U128Box {
    fn from(number: u128) -> U128Box {
        let low = number as u64;
        let high = (number >> 64) as u64;
        U128Box { high, low }
    }
}

#[test]
pub fn uint128_max() {
    println!("u128 max: {:?}", std::u128::MAX);
    let number = U128Box::from(std::u128::MAX);
    assert_eq!(number.get(), std::u128::MAX)
}

#[test]
pub fn uint128_min() {
    println!("u128 min: {:?}", std::u128::MIN);
    let number = U128Box::from(std::u128::MIN);
    assert_eq!(number.get(), std::u128::MIN)
}
