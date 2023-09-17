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

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::ExprArray;
use syn::ItemStruct;
use syn::LitStr;
use syn::Token;

#[allow(dead_code)]
pub struct EntityMacroInput {
    pub(crate) from_ident: Ident,
    pub(crate) equal1: Token![=],
    pub(crate) from_lit: LitStr,
    pub(crate) comma1: Token![,],
    pub(crate) include_ident: Ident,
    pub(crate) equal2: Token![=],
    pub(crate) include_array: ExprArray,
    pub(crate) comma2: Token![,],
    pub(crate) omit_ident: Ident,
    pub(crate) equal3: Token![=],
    pub(crate) omit_array: ExprArray,
}

impl Parse for EntityMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            from_ident: input.parse()?,
            equal1: input.parse()?,
            from_lit: input.parse()?,
            comma1: input.parse()?,
            include_ident: input.parse(),
            equal2: input.parse()?,
            include_array: input.parse(),
            comma2: input.parse()?,
            omit_ident: input.parse()?,
            equal3: input.parse()?,
            omit_array: input.parse()?,
        })
    }
}

pub fn implement_entity(_: &EntityMacroInput, _: &ItemStruct) -> Option<TokenStream> {
    todo!()
}
