/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

use crate::{
    diff::Diff,
    mismatch::{MismatchError, MismatchType},
    patchable::PatchableField,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///TODO: make debug, eq, partialeq, copy, hash optional
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyDiff<T>
    where T: PatchableField {
    new_value: T,
    /// The old value.
    ///
    /// A safety check. A patch cleanly applies when the objects current
    /// value matches the patches `old_value`.
    ///
    /// The alternative `force_…()` functions circumvent this safety check.
    old_value: T,
}

impl<T> Diff for CopyDiff<T>
    where T: Clone + std::cmp::PartialEq + PatchableField + std::fmt::Debug
{
    type DiffResult = Self::Object;
    type Object = T;

    fn new(old: &T, new: &T) -> Self
        where T: Clone {
        CopyDiff { old_value: old.clone(),
                   new_value: new.clone(), }
    }

    /// small helper to actually check if this PatchField changes the obj
    ///
    /// returns true when `old_value == new_value`
    fn changes_object(&self, old: &Self::Object) -> bool {
        self.contains_change() && old.compare(&self.new_value)
    }

    fn contains_change(&self) -> bool {
        !(self.new_value.compare(&self.old_value))
    }

    /// Applies the given `Diff` ontop of this on.
    /// That means that `a.merge(b)` changes`a` so that it represents a change
    /// as if `a` and `b` have been applied consecutevly.Eq
    ///
    /// #
    fn merge(&mut self, rhs: &Self) -> Result<(), ()> {
        if rhs.old_value.compare(&self.new_value) {
            self.new_value = rhs.new_value.clone();
            Ok(())
        } else {
            Err(())
        }
    }

    fn applies_cleanly(&self, obj: &T) -> Result<(), MismatchError>
        where T: std::fmt::Debug {
        if self.old_value.compare(obj) {
            Ok(())
        } else {
            Err(MismatchError::new("food",
                                   format!("{:?}", self.old_value),
                                   format!("{:?}", obj),
                                   MismatchType::PatchOldValue)) //todo:  better error handlings
        }
    }

    /// Applies this patched field onto the given object.
    ///
    /// # Parameters
    /// `obj`: The object where this patch is applied to.
    ///
    /// # Safety
    /// First checks `applies_cleanly` if this would apply cleanly
    /// Returns an Error in case that check fails.
    fn apply_into(&self, obj: &mut T) -> Result<(), MismatchError> {
        match self.applies_cleanly(obj) {
            Ok(()) => {
                *obj = self.new_value.clone();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert::tests::*, patchable::PatchableField};

    struct PatchableStruct {
        pub d1: u64,
        pub d2: String,
    }

    impl PatchableField for PatchableStruct {}

    #[test]
    fn test_sync() {
        assert_sync::<CopyDiff<i8>>();
        assert_sync::<CopyDiff<u8>>();
        assert_sync::<CopyDiff<i16>>();
        assert_sync::<CopyDiff<u16>>();
        assert_sync::<CopyDiff<i32>>();
        assert_sync::<CopyDiff<u32>>();
        assert_sync::<CopyDiff<i64>>();
        assert_sync::<CopyDiff<u64>>();
        assert_sync::<CopyDiff<i128>>();
        assert_sync::<CopyDiff<u128>>();
        assert_sync::<CopyDiff<isize>>();
        assert_sync::<CopyDiff<usize>>();

        assert_sync::<CopyDiff<bool>>();

        assert_sync::<CopyDiff<char>>();

        assert_sync::<CopyDiff<PatchableStruct>>();
    }

    #[test]
    fn test_send() {
        assert_send::<CopyDiff<i8>>();
        assert_send::<CopyDiff<u8>>();
        assert_send::<CopyDiff<i16>>();
        assert_send::<CopyDiff<u16>>();
        assert_send::<CopyDiff<i32>>();
        assert_send::<CopyDiff<u32>>();
        assert_send::<CopyDiff<i64>>();
        assert_send::<CopyDiff<u64>>();
        assert_send::<CopyDiff<i128>>();
        assert_send::<CopyDiff<u128>>();
        assert_send::<CopyDiff<isize>>();
        assert_send::<CopyDiff<usize>>();

        assert_send::<CopyDiff<bool>>();

        assert_send::<CopyDiff<char>>();

        assert_send::<CopyDiff<PatchableStruct>>();
    }
}
