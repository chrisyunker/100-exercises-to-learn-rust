// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl Add<WrappingU32> for WrappingU32 {
    type Output = WrappingU32;
    fn add(self, rhs: WrappingU32) -> Self::Output {
        WrappingU32 {
            value: self.value + rhs.value
        }
    }
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
