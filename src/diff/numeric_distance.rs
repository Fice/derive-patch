/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

use crate::{diff::Diff, mismatch::MismatchError};
use num::Zero;

#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[cfg(not(feature = "serde"))]
pub trait Serialize {}
#[cfg(not(feature = "serde"))]
impl<T> Serialize for T {}
#[cfg(not(feature = "serde"))]
pub trait DeserializeOwned {}
#[cfg(not(feature = "serde"))]
impl<T> DeserializeOwned for T {}


/// todo: make debug optional
/// todo: make PartialEq and Eq optional
#[derive(Debug, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NumericDistanceDiff<T>
    where T: std::ops::Sub<T> + Copy + std::ops::Add<<T as std::ops::Sub>::Output, Output = T>,
          <T as std::ops::Sub>::Output:
              num::Zero + Clone + Copy + std::fmt::Debug + PartialEq + Serialize + DeserializeOwned
{
    difference: <T as std::ops::Sub>::Output,
}
//todo: relax on copy requirement?
impl<T> Diff for NumericDistanceDiff<T>
    where T: std::ops::Sub<T> + Copy + std::ops::Add<<T as std::ops::Sub>::Output, Output = T>,
          <T as std::ops::Sub>::Output:
              num::Zero + Clone + Copy + std::fmt::Debug + PartialEq + Serialize + DeserializeOwned
{
    type DiffResult = <T as std::ops::Sub>::Output;
    type Object = T;

    fn new(old: &T, new: &T) -> Self {
        NumericDistanceDiff {
            difference: (*new) - (*old), // TODO: checked version!
        }
    }

    /// The same as `contains_change` for this Diff Type
    fn changes_object(&self, _old: &Self::Object) -> bool {
        self.contains_change()
    }

    fn contains_change(&self) -> bool
        where Self::DiffResult: num::Zero {
        !self.difference.is_zero()
    }

    fn applies_cleanly(&self, _obj: &T) -> Result<(), MismatchError> {
        Ok(())
    }

    fn apply_into(&self, obj: &mut T) -> Result<(), MismatchError>
        where T: std::ops::Add<<T as std::ops::Sub>::Output, Output = T> + Copy,
              <T as std::ops::Sub>::Output: Copy {
        *obj = (*obj) + (self.difference);
        Ok(())
    }

    fn merge(&mut self, rhs: &Self) -> Result<(), ()> {
        self.difference = self.difference + rhs.difference;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert::tests::*;

    #[test]
    fn test_sync() {
        assert_sync::<NumericDistanceDiff<i8>>();
        assert_sync::<NumericDistanceDiff<u8>>();
        assert_sync::<NumericDistanceDiff<i16>>();
        assert_sync::<NumericDistanceDiff<u16>>();
        assert_sync::<NumericDistanceDiff<i32>>();
        assert_sync::<NumericDistanceDiff<u32>>();
        assert_sync::<NumericDistanceDiff<i64>>();
        assert_sync::<NumericDistanceDiff<u64>>();
        assert_sync::<NumericDistanceDiff<i128>>();
        assert_sync::<NumericDistanceDiff<u128>>();
        assert_sync::<NumericDistanceDiff<isize>>();
        assert_sync::<NumericDistanceDiff<usize>>();

        assert_sync::<NumericDistanceDiff<f32>>();
        assert_sync::<NumericDistanceDiff<f64>>();
    }

    #[test]
    fn test_send() {
        assert_send::<NumericDistanceDiff<i8>>();
        assert_send::<NumericDistanceDiff<u8>>();
        assert_send::<NumericDistanceDiff<i16>>();
        assert_send::<NumericDistanceDiff<u16>>();
        assert_send::<NumericDistanceDiff<i32>>();
        assert_send::<NumericDistanceDiff<u32>>();
        assert_send::<NumericDistanceDiff<i64>>();
        assert_send::<NumericDistanceDiff<u64>>();
        assert_send::<NumericDistanceDiff<i128>>();
        assert_send::<NumericDistanceDiff<u128>>();
        assert_send::<NumericDistanceDiff<isize>>();
        assert_send::<NumericDistanceDiff<usize>>();

        assert_send::<NumericDistanceDiff<f32>>();
        assert_send::<NumericDistanceDiff<f64>>();
    }
}
