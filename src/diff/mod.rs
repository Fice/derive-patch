/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */


//! This module contains the trait for `Diff` and some generic implementations.

mod copy;
mod numeric_distance;
mod traits;

pub use copy::CopyDiff;
pub use numeric_distance::NumericDistanceDiff;
pub use traits::Diff;
