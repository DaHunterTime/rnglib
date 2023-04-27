use std::ops::{Add, Sub, Range, RangeInclusive, RangeFrom, RangeTo, RangeToInclusive, RangeFull};

/// The `ValidRandomNumber` trait.
/// 
/// It defines what makes a type or struct a valid random number.
/// 
/// Currently implemented for:
/// * u32
/// * u64
/// * u128
pub trait ValidRandomNumber: Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> {
    /// Returns the representation of a zero for the given type.
    fn zero() -> Self;

    /// Returns the representation of a one for the given type.
    fn one() -> Self;

    /// Returns the maximum value representation.
    fn max() -> Self;

    /// Returns the maximum amount of a byte as the given type. i.e. what is the equivalent of 255
    /// of your type.
    fn byte_max() -> Self;

    /// Returns a convertion to `u8`.
    fn to_u8(self) -> u8;

    /// Returns a convertion to `usize`.
    fn to_usize(self) -> usize;

    /// Returns a convertion to `f64`.
    fn to_f64(self) -> f64;

    /// Creates this type from a `usize` value.
    fn from_usize(value: usize) -> Self;
}

impl ValidRandomNumber for u32 {
    fn zero() -> u32 {
        return 0;
    }

    fn one() -> u32 {
        return 1;
    }

    fn max() -> u32 {
        return u32::MAX;
    }

    fn byte_max() -> u32 {
        return 255;
    }

    fn to_u8(self) -> u8 {
        return self as u8;
    }

    fn to_usize(self) -> usize {
        return self as usize;
    }

    fn to_f64(self) -> f64 {
        return self as f64;
    }

    fn from_usize(value: usize) -> u32 {
        return value as u32;
    }
}

impl ValidRandomNumber for u64 {
    fn zero() -> u64 {
        return 0;
    }

    fn one() -> u64 {
        return 1;
    }

    fn max() -> u64 {
        return u64::MAX;
    }

    fn byte_max() -> u64 {
        return 255;
    }

    fn to_u8(self) -> u8 {
        return self as u8;
    }

    fn to_usize(self) -> usize {
        return self as usize;
    }

    fn to_f64(self) -> f64 {
        return self as f64;
    }

    fn from_usize(value: usize) -> u64 {
        return value as u64;
    }
}

impl ValidRandomNumber for u128 {
    fn zero() -> u128 {
        return 0;
    }

    fn one() -> u128 {
        return 1;
    }

    fn max() -> u128 {
        return u128::MAX;
    }

    fn byte_max() -> u128 {
        return 255;
    }

    fn to_u8(self) -> u8 {
        return self as u8;
    }

    fn to_usize(self) -> usize {
        return self as usize;
    }

    fn to_f64(self) -> f64 {
        return self as f64;
    }

    fn from_usize(value: usize) -> u128 {
        return value as u128;
    }
}

/// The `ValidRandomRange` trait.
/// 
/// It defines what makes a struct a valid range to use with a type that implements
/// `ValidRandomNumber`.
/// 
/// Currently implemented for all `Range` structs.
pub trait ValidRandomRange<T: ValidRandomNumber> {
    /// Returns the starting value of the range.
    fn _start(&self) -> T;

    /// Returns the last value of the range.
    fn _end(&self) -> T;
}

impl<T: ValidRandomNumber> ValidRandomRange<T> for Range<T> {
    fn _start(&self) -> T {
        return self.start;
    }

    fn _end(&self) -> T {
        return self.end;
    }
}

impl<T: ValidRandomNumber> ValidRandomRange<T> for RangeInclusive<T> {
    fn _start(&self) -> T {
        return *self.start();
    }

    fn _end(&self) -> T {
        return *self.end() + T::one();
    }
}

impl<T: ValidRandomNumber> ValidRandomRange<T> for RangeFrom<T> {
    fn _start(&self) -> T {
        return self.start;
    }

    fn _end(&self) -> T {
        return T::max();
    }
}

impl<T: ValidRandomNumber> ValidRandomRange<T> for RangeTo<T> {
    fn _start(&self) -> T {
        return T::zero();
    }

    fn _end(&self) -> T {
        return self.end;
    }
}

impl<T: ValidRandomNumber> ValidRandomRange<T> for RangeToInclusive<T> {
    fn _start(&self) -> T {
        return T::zero();
    }

    fn _end(&self) -> T {
        return self.end;
    }
}

impl<T: ValidRandomNumber> ValidRandomRange<T> for RangeFull {
    fn _start(&self) -> T {
        return T::zero();
    }

    fn _end(&self) -> T {
        return T::max();
    }
}
