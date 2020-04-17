/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

//! Structures for storing and dealing with a missmatch that prevents
//! a `patch` or `partial` from applying cleanly.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{convert::Into, error::Error, fmt};

/// Functions that require a complete `Patch` or `Partial` return this Error in
/// case the operation failed due to an incomplete Object
///
/// See `Patch::is_complete()` and `Partial::_is_complete()`
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
//todo: #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IncompleteError<Incompletable> {
    operation: &'static str,
    object_id: String,
    incomplete: Incompletable,
}
impl<Incompletable> IncompleteError<Incompletable> {
    /// creates a new IncompleteError
    pub fn new<T>(
        operation: &'static str,
        object_id: T,
        incomplete: Incompletable,
    ) -> IncompleteError<Incompletable>
    where
        T: std::convert::Into<String>,
    {
        IncompleteError {
            operation,
            object_id: object_id.into(),
            incomplete,
        }
    }
}
impl<Incompletable> fmt::Display for IncompleteError<Incompletable>
where
    Incompletable: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IncompleteError: `{}` failed for {}.\nData:\n{:?}",
            self.operation, self.object_id, self.incomplete
        )
    }
}

impl<Incompletable> Error for IncompleteError<Incompletable> where Incompletable: std::fmt::Debug {}

/// Holds information about the mismatch that occured
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MismatchError {
    field_name: &'static str,
    /// The expected value for the operation to succeed
    pub expected: String,
    /// The actual value during the operation
    ///
    /// Must be `!=` to `expected`.
    pub received: String,

    mismatch_type: MismatchType,
}
impl MismatchError {
    /// creates a new MismatchData
    pub fn new<E, R>(
        field_name: &'static str,
        expected: E,
        received: R,
        mismatch_type: MismatchType,
    ) -> MismatchError
    where
        E: Into<String>,
        R: Into<String>,
    {
        MismatchError {
            field_name,
            expected: expected.into(),
            received: received.into(),
            mismatch_type,
        }
    }

    /// getter for the field name
    pub fn name(&self) -> &'static str {
        self.field_name
    }
}
impl fmt::Display for MismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} - field {}: expected {}, got {}",
            self.mismatch_type, self.field_name, self.expected, self.received
        )
    }
}

/// To differentiate between the different types of mismatches that can happen
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MismatchType {
    /// The patch could not apply, because an id field mismatched
    ObjectID,
    /// The patch could not apply, because the expected value does not match the
    /// `old_value` of the patch
    PatchOldValue,
}
impl fmt::Display for MismatchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match self {
            MismatchType::ObjectID => "Object id didn't match patch id",
            MismatchType::PatchOldValue => "Current object valie did not match old patch value",
        };
        write!(f, "{}", desc)
    }
}

/// Funcgtions that require specific fields to be a specific value can return
/// this error in case the value differed.
///
/// There are two types of mismatches that can hapen:
/// - because an id field mismatched
/// - because the expected value does not match the `old_value` of the patch
#[derive(Debug, Clone, Eq, PartialEq, Default)]
//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MultipleMismatchError {
    //#[serde(borrow = "'a")]
    mismatches: Vec<MismatchError>,
}
impl MultipleMismatchError {
    /// creates a new mismatch error
    ///
    /// to actually add the mismatches, use `MismatchError::add_error()`
    pub fn new() -> MultipleMismatchError {
        Default::default()
    }

    /// function to actually add a mismatch error
    ///
    /// `is_error_free()` return true until the first call to this function.
    /// Afterwards `is_error_free()` returns true.
    pub fn add_error(&mut self, value: MismatchError) {
        self.mismatches.push(value);
    }

    /// returns true when there are *no* actual mismatches stored inside
    ///
    /// also see `has_errors` for the negated function.
    pub fn is_error_free(&self) -> bool {
        self.mismatches.is_empty()
    }

    /// returns true when there are actual mismatches stored inside
    ///
    /// also see `is_error_free` for the negated function.
    pub fn has_errors(&self) -> bool {
        !self.mismatches.is_empty()
    }

    /// merges the errors of both MultipleMismatchError into one
    pub fn merge(&mut self, rhs: &MultipleMismatchError) {
        self.mismatches.extend(rhs.mismatches.iter().cloned())
    }
}

impl Error for MultipleMismatchError {}

impl fmt::Display for MultipleMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mismatches = self.mismatches.iter().fold(String::new(), |acc, mismatch| {
            format!("{}{}\n", acc, mismatch)
        });

        write!(f, "{} mismatches:\n{}", self.mismatches.len(), mismatches)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::assert::tests::*;

    #[derive(Debug)]
    struct DummyIncompletable {}

    #[test]
    fn test_sync() {
        assert_sync::<MultipleMismatchError>();
        assert_sync::<MismatchType>();
        assert_sync::<IncompleteError<DummyIncompletable>>();
    }

    #[test]
    fn test_send() {
        assert_send::<MultipleMismatchError>();
        assert_send::<MismatchType>();
        assert_send::<IncompleteError<DummyIncompletable>>();
    }
}
