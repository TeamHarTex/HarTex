/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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
#![feature(let_chains)]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::ItemStruct;
use syn::parse_macro_input;

mod commandmetadata;
mod pluginmetadata;

/// Macro to implement the `CommandMetadata` trait.
#[proc_macro_attribute]
pub fn command(tokens: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as commandmetadata::CommandMetadataMacroInput);
    let struct_decl = parse_macro_input!(item as ItemStruct);
    commandmetadata::implement_metadata(&input, &struct_decl)
        .unwrap_or_default()
        .into()
}

/// Macro to implement the `PluginMetadata` trait.
#[proc_macro_attribute]
pub fn plugin(tokens: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as pluginmetadata::PluginMetadataMacroInput);
    let struct_decl = parse_macro_input!(item as ItemStruct);
    pluginmetadata::implement_metadata(&input, &struct_decl)
        .unwrap_or_default()
        .into()
}
