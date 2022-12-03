/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

#![deny(clippy::pedantic)]
#![deny(warnings)]
#![feature(drain_filter)]
#![feature(let_chains)]
#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_quote)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Error};

mod commandmetadata;

#[proc_macro_derive(CommandMetadata, attributes(metadata))]
pub fn derive_command_metadata_trait(tokens: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(tokens as DeriveInput);
    commandmetadata::expand_command_metadata_derivation(&mut input)
        .unwrap_or_else(as_compiler_errors)
        .into()
}

#[allow(clippy::needless_pass_by_value)]
fn as_compiler_errors(errors: Vec<Error>) -> TokenStream2 {
    let compiler_errors = errors.iter().map(Error::to_compile_error);
    quote::quote!(#(#compiler_errors)*)
}
