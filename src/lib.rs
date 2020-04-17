/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

//! This crate contains 2 macros.
//!
//! One for generating a `partial` Variant of an object. (Inspired by typescript
//! ```Partial<Object>```)
//!
//! If you want a peek at what the generated struct should to look like, have a
//! look at tests/template_partial.rs and tests/template_patch.rs

//#![feature(associated_type_defaults)]

#![deny(missing_docs)]

extern crate macro_impl;

#[cfg(feature = "serde")]
extern crate serde;

pub mod mismatch;
pub mod patchable;
pub mod traits;

pub mod diff;

#[cfg(test)]
pub(crate) mod assert;
