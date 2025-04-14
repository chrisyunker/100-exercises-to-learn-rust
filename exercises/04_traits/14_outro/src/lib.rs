// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::{ops::Add, u16};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self { value: value as u16 }
    }
}
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { value: *value as u16 }
    }
}
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self { value: *value as u16 }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
    fn ne(&self, other: &u16) -> bool {
        self.value != *other
    }
}

impl Add for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: Self) -> Self::Output {
        if self.value as u32 + rhs.value as u32  > u16::MAX as u32 {
            Self { value: u16::MAX }
        }  else {
            Self { value: self.value + rhs.value }
        }
    }
}
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        if self.value as u32 + rhs as u32 > u16::MAX as u32 {
            Self { value: u16::MAX }
        }  else {
            Self { value: self.value + rhs }
        }
    }
}
impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        if self.value as u32 + *rhs as u32 > u16::MAX as u32 {
            Self { value: u16::MAX }
        }  else {
            Self { value: self.value + *rhs }
        }
    }
}
impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        if self.value as u32 + rhs.value as u32 > u16::MAX as u32 {
            Self { value: u16::MAX }
        }  else {
            Self { value: self.value + rhs.value }
        }
    }
}