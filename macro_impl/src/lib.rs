/*
 * Copyright (2020) by Marcel Lambert.
 * This project's License is the MIT Open Source license.
 * For more information, see the LICENSE.md file in this repository.
 */

//! The actual macro implementation for `derive-patch`.
//! Do not use directly.

#![deny(missing_docs)]

use proc_macro::TokenStream;

/// todo:
#[proc_macro_derive(Partial)]
pub fn partial(input: TokenStream) -> TokenStream {
    input
}

/// todo:
#[proc_macro_derive(Patch)]
pub fn patch(input: TokenStream) -> TokenStream {
    input
}
