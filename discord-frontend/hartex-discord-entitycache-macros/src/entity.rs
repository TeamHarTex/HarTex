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
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
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
use syn::Type;

use crate::metadata;

const PRELUDE_AND_PRIMITIVES: [&str; 21] = [
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "&str",
    "bool", "char", "f32", "f64", "Option", "Box", "String", "Vec",
];

// FIXME: this needs to be reimagined
#[allow(dead_code)]
#[allow(clippy::module_name_repetitions)]
pub struct EntityMacroInput {
    from_ident: Ident,
    equal1: Token![=],
    from_lit_str: LitStr,
    comma1: Token![,],
    id_ident: Ident,
    equal3: Token![=],
    id_array: ExprArray,
    comma3: Token![,],
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
            id_ident: input.parse()?,
            equal3: input.parse()?,
            id_array: input.parse()?,
            comma3: input.parse()?,
            exclude_or_include_ident: input.parse()?,
            equal2: input.parse()?,
            exclude_or_include_array: input.parse()?,
            comma2: input.parse().ok(),
        })
    }
}

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::too_many_lines)]
pub fn implement_entity(input: &EntityMacroInput, item_struct: &ItemStruct) -> Option<TokenStream> {
    if input.from_ident != "from" {
        input
            .from_ident
            .span()
            .unwrap()
            .error("expected `from`")
            .emit();

        return None;
    }

    if input.id_ident != "id" {
        input.id_ident.span().unwrap().error("expected `id`").emit();
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

    let type_metadata = metadata::STRUCT_MAP
        .get(type_key.as_str())
        .copied()
        .cloned()
        .unwrap();
    let mut any_not_found = false;
    let fields = input
        .exclude_or_include_array
        .elems
        .iter()
        .filter_map(|expr| match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                if type_metadata
                    .fields
                    .iter()
                    .any(|field| field.name == lit_str.value())
                {
                    Some(lit_str.value())
                } else {
                    lit_str
                        .span()
                        .unwrap()
                        .error(format!("field `{}` cannot be found in type `{type_key}`", lit_str.value()))
                        .note(format!(
                            "the type metadata generated was for twilight-model version {}",
                            metadata::CRATE_VERSION
                        ))
                        .help("consider regenerating the metadata for a newer version if the field is recently added")
                        .emit();
                    any_not_found = true;

                    None
                }
            }
            expr => {
                expr.span()
                    .unwrap()
                    .warning("non-string expressions are ignored")
                    .emit();

                None
            }
        })
        .collect::<Vec<_>>();

    let id_fields = input
        .id_array
        .elems
        .iter()
        .filter_map(|expr| match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                if type_metadata
                    .fields
                    .iter()
                    .any(|field| field.name == lit_str.value())
                {
                    Some(lit_str.value())
                } else {
                    lit_str
                        .span()
                        .unwrap()
                        .error(format!("field `{}` cannot be found in type `{type_key}`", lit_str.value()))
                        .note(format!(
                            "the type metadata generated was for twilight-model version {}",
                            metadata::CRATE_VERSION
                        ))
                        .help("consider regenerating the metadata for a newer version if the field is recently added")
                        .emit();
                    any_not_found = true;

                    None
                }
            }
            expr => {
                expr.span()
                    .unwrap()
                    .warning("non-string expressions are ignored")
                    .emit();

                None
            }
        })
        .collect::<Vec<_>>();

    if any_not_found {
        return None;
    }

    let item_struct_vis = item_struct.vis.clone();
    let item_struct_name = item_struct.ident.clone();

    let mut fields_tokens = match input.exclude_or_include_ident.to_string().as_str() {
        "exclude" => type_metadata
            .fields
            .iter()
            .filter_map(|field| {
                if fields.contains(&field.name) {
                    None
                } else {
                    let field_name = Ident::new(field.name.as_str(), Span::call_site());
                    let field_type = syn::parse_str::<Type>(
                        expand_fully_qualified_type_name(field.ty.clone()).as_str(),
                    )
                    .unwrap();

                    Some(quote! {#field_name: #field_type})
                }
            })
            .collect::<Vec<_>>(),
        "include" => type_metadata
            .fields
            .iter()
            .filter_map(|field| {
                if fields.contains(&field.name) {
                    let field_name = Ident::new(field.name.as_str(), Span::call_site());
                    let field_type = syn::parse_str::<Type>(
                        expand_fully_qualified_type_name(field.ty.clone()).as_str(),
                    )
                    .unwrap();

                    Some(quote! {#field_name: #field_type})
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        _ => {
            input
                .exclude_or_include_ident
                .span()
                .unwrap()
                .error("expected `exclude` or `include`")
                .emit();

            return None;
        }
    };

    fields_tokens.append(
        &mut type_metadata
            .fields
            .iter()
            .filter_map(|field| {
                if id_fields.contains(&field.name) {
                    let field_name = Ident::new(field.name.as_str(), Span::call_site());
                    let field_type = syn::parse_str::<Type>(
                        expand_fully_qualified_type_name(field.ty.clone()).as_str(),
                    )
                    .unwrap();

                    Some(quote! {#field_name: #field_type})
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
    );

    Some(quote! {
        #item_struct_vis struct #item_struct_name {
            #(#fields_tokens),*
        }
    })
}

fn expand_fully_qualified_type_name(mut to_expand: String) -> String {
    to_expand = to_expand.replace(' ', "");

    let open_angle_brackets = to_expand.find('<');
    let close_angle_brackets = to_expand.rfind('>');

    if open_angle_brackets.is_none() && close_angle_brackets.is_none() {
        if PRELUDE_AND_PRIMITIVES.contains(&to_expand.as_str()) {
            return to_expand;
        }

        let Some(fully_qualified) = metadata::STRUCT_MAP.keys().find(|key| {
            let index = key.rfind(':').unwrap();
            key[index + 1..] == to_expand
        }) else {
            return to_expand;
        };

        return (*fully_qualified).to_string();
    }

    format!(
        "{}<{}>",
        expand_fully_qualified_type_name(to_expand[0..open_angle_brackets.unwrap()].to_string()),
        expand_fully_qualified_type_name(
            to_expand[open_angle_brackets.unwrap() + 1..close_angle_brackets.unwrap()].to_string()
        )
    )
}
