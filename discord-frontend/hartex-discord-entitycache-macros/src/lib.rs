/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

//! # Entity Cache Macros
//!
//! Useful macros for the entity cache.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(clippy::too_many_lines)]
#![feature(let_chains)]
#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::ItemStruct;

mod entity;
#[path = "../generated/metadata.rs"]
mod metadata;
mod reflect;
mod typeext;

/// Implementatioon of the `entity` attribute macro.
#[proc_macro_attribute]
pub fn entity(tokens: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as entity::EntityMacroInput);
    let struct_decl = parse_macro_input!(item as ItemStruct);
    entity::implement_entity(&input, &struct_decl)
        .unwrap_or_default()
        .into()
}
