/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

//! Contains traits

use crate::mismatch::MultipleMismatchError;

/// Extension trait that all structs S with `patch!(struct S {})` implement.
pub trait PatchableExt
where
    Self: Sized,
{
    // get_patch(self, rhs: Self) -> ;
    // get_patch_from_partial(self, rhs: Partial)
}

/// Trait for everything Patch and Partial have in common
pub trait Base<ErrorType> {
    /// The object type this patch applies to.
    type Target;

    /// The amount of fields that are settable with this patch
    ///
    /// See `count()`
    const MAX_FIELDS: usize;

    /// Are all fields of type `Target`actually set in this Patch.
    /// False if one or more fields are not set
    ///
    /// Also see `is_empty`
    fn is_complete(&self) -> bool;

    /// Are all fields in this patch empty.
    /// True if all fields are empty, false otherwise.
    ///
    /// Also see `is_complete`
    fn is_empty(&self) -> bool;

    /// Counts the number of fields that are not empty.
    ///
    /// Also see `is_complete` and `is_empty`
    ///
    /// Is less or equal to `MAX_FIELDS`
    fn count(&self) -> u32;

    /// applies this patch to the given object.
    ///
    /// # Arguments
    /// - *obj*: The target to change.
    ///
    /// # Safety
    ///
    /// Runs `check()` before manipulating any data.
    fn apply(&self, obj: &mut Self::Target) -> ErrorType;

    // Checks if the same fields have values in both objects.
    //
    // does not care about the actual value set
    /* todo: fn same_fields_set(Self) -> bool; */

    //fn merge_into(&mut self, mergee: Self) -> ErrorType;
}

/// Base Trait for all Partials.
pub trait Partial: Base<()> + std::convert::TryFrom<<Self as Base<()>>::Target> {}

/// Trait for all Patches
pub trait Patch: Base<Result<(), MultipleMismatchError>> {
    /// removes useless patch fields (e.g. old value == new value)
    /// Calls `Diff::contains_change()` on every field. See the concrete Diff
    /// implementation for more Information.
    ///
    /// If this function did clean up fields it will return true.
    fn cleanup(&mut self) -> bool;

    /// checks all the id field of this patch and makes sure they equal obj.
    ///
    /// If at least one id field mismatched, this function returns a
    /// `MultipleMismatchError` with all fields that did not match.
    ///
    /// # Attributes
    /// - *obj*: The object to check.
    ///
    /// Does not check the if the patch applies cleanly, see `check()` for that.
    fn is_correct_target(&self, obj: &Self::Target) -> Result<(), MultipleMismatchError>;

    /// todo: move this function and 'is_correct_target' into special IDPatch
    /// trait
    fn is_same_target(&self, other: &Self) -> bool;

    /// Returns true if this patch would apply without conflicts on the provided
    /// object.
    ///
    /// Calls `Diff::applies_cleanly()` on all non-empty fields.
    ///
    /// Does not check the target, see `check()` for that.
    fn can_apply_cleanly(&self, obj: &Self::Target) -> Result<(), MultipleMismatchError>;

    /// Checks both `is_correct_target` and `can_apply_cleanly`.
    ///
    /// returns MultipleMismatchError that contains errors of both functions
    /// combined.s
    fn check(&self, obj: &Self::Target) -> Result<(), MultipleMismatchError> {
        let e1 = self.is_correct_target(obj);
        let e2 = self.can_apply_cleanly(obj);

        match (e1, e2) {
            (Ok(_), Ok(_)) => Ok(()),
            (Err(e), Ok(_)) => Err(e),
            (Ok(_), Err(e)) => Err(e),
            (Err(mut e1), Err(e2)) => {
                e1.merge(&e2);
                Err(e1)
            }
        }
    }
}
