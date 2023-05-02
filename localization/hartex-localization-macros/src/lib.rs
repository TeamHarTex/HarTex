/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # Localization Macros

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse_macro_input;

use crate::types::Parameters;
use crate::types::ParametersWithArgs;

mod expand_bundle_get;
mod expand_bundle_get_args;
mod types;

/// Obtain a value from a bundle using its key.
#[proc_macro]
pub fn bundle_get(tokens: TokenStream) -> TokenStream {
    let parameters = parse_macro_input!(tokens as Parameters);
    expand_bundle_get::expand_bundle_get(parameters)
        .unwrap_or(TokenStream2::new())
        .into()
}

/// Obtain a value from a bundle using its key, with arguments.
#[proc_macro]
pub fn bundle_get_args(tokens: TokenStream) -> TokenStream {
    let parameters = parse_macro_input!(tokens as ParametersWithArgs);
    expand_bundle_get_args::expand_bundle_get_args(parameters)
        .unwrap_or(TokenStream2::new())
        .into()
}
