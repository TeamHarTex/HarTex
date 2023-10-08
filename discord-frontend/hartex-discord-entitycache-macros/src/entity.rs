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
use syn::Expr;
use syn::ExprArray;
use syn::ExprLit;
use syn::ItemStruct;
use syn::Lit;
use syn::LitStr;
use syn::Token;

use crate::metadata;

// FIXME: this needs to be reimagined
#[allow(dead_code)]
pub struct EntityMacroInput {
    from_ident: Ident,
    equal1: Token![=],
    from_lit_str: LitStr,
    comma1: Token![,],
    exclude_or_include_ident: Ident,
    equal2: Token![=],
    exclude_or_include_array: ExprArray,
    comma2: Option<Token![,]>,
}

impl Parse for EntityMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            from_ident: input.parse()?,
            equal1: input.parse()?,
            from_lit_str: input.parse()?,
            comma1: input.parse()?,
            exclude_or_include_ident: input.parse()?,
            equal2: input.parse()?,
            exclude_or_include_array: input.parse()?,
            comma2: input.parse().ok(),
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

    let type_key = input.from_lit_str.value();
    if !metadata::STRUCT_MAP.contains_key(type_key.as_str()) {
        input
            .from_lit_str
            .span()
            .unwrap()
            .error(format!("type `{type_key}` cannot be found"))
            .note(format!(
                "the type metadata generated was for twilight-model version {}",
                metadata::CRATE_VERSION
            ))
            .help("consider regenerating the metadata for a newer version if the type is recently added")
            .emit();

        return None;
    }

    let _ = input
        .exclude_or_include_array
        .elems
        .iter()
        .filter_map(|expr| match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => Some(lit_str.value()),
            expr => {
                expr.span()
                    .unwrap()
                    .warning("non-string expressions are ignored")
                    .emit();

                None
            }
        })
        .collect::<Vec<_>>();

    match input.exclude_or_include_ident.to_string().as_str() {
        "exclude" => None,
        "include" => None,
        _ => {
            input
                .exclude_or_include_ident
                .span()
                .unwrap()
                .error("expected `exclude` or `include`")
                .emit();

            return None;
        }
    }
}
