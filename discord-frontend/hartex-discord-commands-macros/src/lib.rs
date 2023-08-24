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

//! # Command System Macros
//!
//! This ctrate provides certain macros for the command system that may otherwise be useful.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(extract_if)]
#![feature(let_chains)]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::ItemStruct;

mod commandmetadata;

/// Macro to derive the `CommandMetadata` trait.
///
/// Deprecated since **0.3.0**. Use the `#[metadata]` attribute macro instead.
#[allow(deprecated)]
#[deprecated(since = "0.3.0")]
#[proc_macro_derive(CommandMetadata, attributes(metadata))]
pub fn derive_command_metadata_trait(tokens: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(tokens as DeriveInput);
    commandmetadata::expand_command_metadata_derivation(&mut input)
        .unwrap_or_default()
        .into()
}

/// Macro to implement the `CommandMetadata` trait.
#[proc_macro_attribute]
pub fn metadata(tokens: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(tokens as commandmetadata::MetadataMacroInput);
    let mut struct_decl = parse_macro_input!(tokens as ItemStruct);
    commandmetadata::implement_metadata(&mut input, &mut struct_decl)
        .unwrap_or_default()
        .into()
}
