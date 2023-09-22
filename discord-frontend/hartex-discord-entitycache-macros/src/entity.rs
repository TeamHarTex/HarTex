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
use syn::spanned::Spanned;
use syn::ExprArray;
use syn::ItemStruct;
use syn::LitStr;
use syn::Token;

use crate::metadata;

#[allow(dead_code)]
pub struct EntityMacroInput {
    pub(crate) from_ident: Ident,
    pub(crate) equal1: Token![=],
    pub(crate) from_lit: LitStr,
    pub(crate) comma1: Token![,],
    pub(crate) exclude_ident: Option<Ident>,
    pub(crate) equal2: Option<Token![=]>,
    pub(crate) exclude_array: Option<ExprArray>,
    pub(crate) comma2: Option<Token![,]>,
    pub(crate) include_ident: Option<Ident>,
    pub(crate) equal3: Option<Token![=]>,
    pub(crate) include_array: Option<ExprArray>,
    pub(crate) comma3: Option<Token![,]>,
    pub(crate) id_ident: Ident,
    pub(crate) equal4: Token![=],
    pub(crate) id_array: ExprArray,
    pub(crate) comma4: Option<Token![,]>,
}

impl Parse for EntityMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            from_ident: input.parse()?,
            equal1: input.parse()?,
            from_lit: input.parse()?,
            comma1: input.parse()?,
            exclude_ident: input.parse()?,
            equal2: input.parse()?,
            exclude_array: input.parse()?,
            comma2: input.parse()?,
            include_ident: input.parse()?,
            equal3: input.parse()?,
            include_array: input.parse()?,
            comma3: input.parse()?,
            id_ident: input.parse()?,
            equal4: input.parse()?,
            id_array: input.parse()?,
            comma4: input.parse().ok(),
        })
    }
}

pub fn implement_entity(input: &EntityMacroInput, _: &ItemStruct) -> Option<TokenStream> {
    if input.from_ident != "from" {
        input
            .from_ident
            .span()
            .unwrap()
            .error("expected `from`")
            .emit();

        return None;
    }

    if input.exclude_ident.is_some() && input.include_array.is_some() {
        input
            .exclude_ident
            .unwrap()
            .span()
            .unwrap()
            .error("the `exclude` and `include` parameters are mutually exclusive")
            .emit();

        input
            .include_array
            .unwrap()
            .span()
            .unwrap()
            .error("the `exclude` and `include` parameters are mutually exclusive")
            .emit();

        return None;
    }

    if input.exclude_array.is_some() && input.include_array.is_some() {
        unreachable!()
    }

    if input.id_ident != "id" {
        input.id_ident.span().unwrap().error("expected `id`").emit();

        return None;
    }

    let Some(struct_metadata) = metadata::STRUCT_MAP.get(input.from_lit.value().as_str()) else {
        input
            .from_lit
            .span()
            .unwrap()
            .error("this struct is not found");

        return None;
    };
    let _ = struct_metadata
        .fields
        .iter()
        .map(|field| field.name.clone())
        .collect::<Vec<_>>();

    if let Some(_) = input.exclude_array.clone() && input.exclude_ident.is_some() {
    } else if let Some(_) = input.include_array.clone() && input.include_ident.is_some() {
    }

    todo!()
}
