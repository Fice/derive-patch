/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

use crate::mismatch::MismatchError;

/// Trait to specify different Diffing Techniques.Eq
/// The idea is, that we don't always have to store are copy of the new value.
///
/// This can save memory when storing part of the data, e.g. for vectors
///
/// It allows us to have nested patches.
///
/// It also allows us different conflict resolution strategies.
pub trait Diff
where
    Self: Sized,
{
    /// The storage type that holds the diff
    type DiffResult: Clone;
    /// The object type we want to diff.
    type Object;

    /// Given the old and the new value, creates a diff.
    ///
    /// # Arguments
    ///
    /// * `obj` - The object to create the diff against.
    /// * `new` - The new value the created diff represents.
    fn new(old: &Self::Object, new: &Self::Object) -> Self;

    /// Checks if the given diff would actually change the object.
    ///
    /// Should check that when diff is actually applied on the value, it would
    /// change.
    ///
    /// # Arguments
    ///
    /// * `old` - the current object value
    /// * `diff` - The diff
    fn changes_object(&self, old: &Self::Object) -> bool;

    /// check if this diff has a potential change.
    ///
    /// NOTE: to find out if we really would change the value, check
    /// `changes_object` which compares the objects current value with the
    /// one this diff woul apply.
    fn contains_change(&self) -> bool;

    /// Applies the given `Diff` ontop of this on.
    /// That means that `a.merge(b)` changes`a` so that it represents a change
    /// as if `a` and `b` have been applied consecutevly.Eq
    fn merge(&mut self, rhs: &Self) -> Result<(), ()>;

    /// checks the obj if this diff applies without a merge conflict.
    fn applies_cleanly(&self, obj: &Self::Object) -> Result<(), MismatchError>;

    /// Applies this `diff` onto the given `object``
    ///
    /// # Arguments
    /// * `obj` - The object to change.
    ///
    /// Should fail, if applies_cleany returns an error!
    fn apply_into(&self, obj: &mut Self::Object) -> Result<(), MismatchError>;
}
