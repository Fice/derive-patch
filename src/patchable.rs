/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

//! Crate dealing will all different kinds of fields that can be `patchable``



/// todo:
pub trait PatchableField
    where Self: Sized {
    /// todo
    fn compare(&self, rhs: &Self) -> bool
        where Self: PartialEq {
        self == rhs
    }

    /// TODO: copy or clone
    fn copy(&self) -> Self
        where Self: Clone {
        self.clone()
    }

    /*fn copy(&self) -> Self
    where Self: Copy {
        self.copy()
    }*/
}

impl PatchableField for u8 {}
impl PatchableField for i8 {}
impl PatchableField for u16 {}
impl PatchableField for i16 {}
impl PatchableField for u32 {}
impl PatchableField for i32 {}
impl PatchableField for u64 {}
impl PatchableField for i64 {}
impl PatchableField for u128 {}
impl PatchableField for i128 {}
impl PatchableField for usize {}
impl PatchableField for isize {}

impl PatchableField for f32 {
    #[cfg(feature = "epsilon_compare")]
    fn compare(&self, rhs: &f32) -> bool {
        (self - rhs).abs() < std::f32::EPSILON
    }
}
impl PatchableField for f64 {
    #[cfg(feature = "epsilon_compare")]
    fn compare(&self, rhs: &f64) -> bool {
        (self - rhs).abs() < std::f64::EPSILON
    }
}

impl PatchableField for bool {}
impl PatchableField for char {}
impl PatchableField for String {}

impl<T> PatchableField for Option<T> where T: PatchableField + Clone {}

impl<O, E> PatchableField for Result<O, E>
    where O: PatchableField + Clone,
          E: PatchableField + Clone
{
}

//impl<T> PatchableField for [T]
//where T: PatchableField {}

impl<T> PatchableField for Vec<T> where T: PatchableField + Clone {}

//TODO: how to deal with enums
//impl PatchableField for enum {}

//TODO:; array

impl<A, B> PatchableField for (A, B)
    where A: PatchableField + Clone,
          B: PatchableField + Clone
{
}

impl<A, B, C> PatchableField for (A, B, C)
    where A: PatchableField + Clone,
          B: PatchableField + Clone,
          C: PatchableField + Clone
{
}

impl<A, B, C, D> PatchableField for (A, B, C, D)
    where A: PatchableField + Clone,
          B: PatchableField + Clone,
          C: PatchableField + Clone,
          D: PatchableField + Clone
{
}


#[cfg(test)]
mod tests {
    use super::*;

    struct PatchableStruct {
        pub d1: u64,
        pub d2: String,
    }

    impl PatchableField for PatchableStruct {}



    #[test]
    fn patchable_compare() {
        assert!(!12u8.compare(&6u8));
        assert!(4u8.compare(&4u8));
    }
}
