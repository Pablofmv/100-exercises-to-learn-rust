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

use std::ops::Add;

impl Add<SaturatingU16> for SaturatingU16 {

    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {

        Self {
            value: self.value.saturating_add(rhs.value)
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {

    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {

        Self {
            value: self.value.saturating_add(rhs.value)
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {

        Self {
            value: self.value.saturating_add(rhs)
        }
    }
}

impl Add<&u16> for SaturatingU16 {

    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {

        Self {
            value: self.value.saturating_add(*rhs)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SaturatingU16 {
    pub value: u16,
}

impl PartialEq<u16> for SaturatingU16 {

    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}




impl From<u16> for SaturatingU16 {

    fn from(x: u16) -> SaturatingU16 {

        SaturatingU16 {
            value: x
        }

    }
}

impl From<&u16> for SaturatingU16 {

    fn from(x: &u16) -> SaturatingU16 {
        
        SaturatingU16 {
            value: *x
        }
    }
}

impl From<u8> for SaturatingU16 {

    fn from(x: u8) -> SaturatingU16 {

        SaturatingU16 {
            value: x as u16
        }
    }
}

impl From<&u8> for SaturatingU16 {

    fn from(x: &u8) -> SaturatingU16 {
        
        SaturatingU16 {
            value: *x as u16
        }
    }
}




////// Porposed Example

use std::ops::Add;

impl Add<ClampedI8> for ClampedI8 {
    type Output = ClampedI8;

    fn add(self, rhs: ClampedI8) -> Self::Output {
        Self {
            value: self.value + rhs.value.clamp(rhs.value as i8, rhs.value as i8)
        }
    }
}

impl Add<i8> for ClampedI8 {
    type Output =  ClampedI8;

    fn add(self, rhs: i8) -> Self::Output {
        Self {
            value: self.value + rhs.clamp(rhs, rhs)
        }
    }
}

impl Add<&i8> for ClampedI8 {
    type Output = ClampedI8;

    fn add(self, rhs: &i8) -> Self::Output {
        Self {
            value: self.value + *rhs.clamp(*rhs, *rhs)
        }
    }
}

impl Add<&ClampedI8> for ClampedI8 {
    type Output =  ClampedI8;

    fn add(self, rhs: &ClampedI8) -> Self::Output {
        Self {
            value: self.value + rhs.value.clamp(rhs.value, rhs.value)
        }
    }
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClampedI8 {
    value: i8,
}

impl PartialEq<i8> for ClampedI8 {

    fn eq(&self, x: &i8) -> bool {
        self.value == *x
    }
}

impl From<i8> for ClampedI8 {
    fn from(x: i8) -> ClampedI8 {
        ClampedI8 {
            value: x
        }
    }
}

impl From<i16> for ClampedI8 {
    fn from(x: i16) -> ClampedI8 {
        ClampedI8 {
            value: x as i8
        }
    }
}

impl From<&i8> for ClampedI8 {
    fn from(x: &i8) -> ClampedI8 {
        ClampedI8 {
            value: *x
        }
    }
}

impl From<&i16> for ClampedI8 {
    fn from(x: &i16) -> ClampedI8 {
        ClampedI8 {
            value: *x as i8
        }
    }
}

