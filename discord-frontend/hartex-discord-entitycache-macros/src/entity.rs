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
use quote::ToTokens;
use syn::bracketed;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Bracket;
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
    comma5: Token![,],
    extra_fields_ident: Ident,
    equal5: Token![=],
    extra_fields_array: KeyValueArray,
    comma2: Option<Token![,]>,
    overrides_ident: Option<Ident>,
    equal4: Option<Token![=]>,
    overrides_array: Option<KeyValueArray>,
    comma4: Option<Token![,]>,
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
            comma5: input.parse()?,
            extra_fields_ident: input.parse()?,
            equal5: input.parse()?,
            extra_fields_array: input.parse()?,
            comma2: input.parse().ok(),
            overrides_ident: input.parse().ok(),
            equal4: input.parse().ok(),
            overrides_array: input.parse().ok(),
            comma4: input.parse().ok(),
        })
    }
}

#[derive(Clone)]
struct KeyValueArray {
    #[allow(dead_code)]
    bracket_token: Bracket,
    elements: Punctuated<KeyValueArrayElement, Token![,]>,
}

impl Parse for KeyValueArray {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);
        let mut elements = Punctuated::new();

        while !content.is_empty() {
            let first = content.parse::<KeyValueArrayElement>()?;
            elements.push_value(first);

            if content.is_empty() {
                break;
            }

            let punct = content.parse()?;
            elements.push_punct(punct);
        }

        Ok(Self {
            bracket_token,
            elements,
        })
    }
}

#[derive(Clone)]
struct KeyValueArrayElement {
    key: LitStr,
    #[allow(dead_code)]
    colon: Token![:],
    value: LitStr,
}

impl Parse for KeyValueArrayElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            colon: input.parse()?,
            value: input.parse()?,
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

        return None;
    }

    if input.extra_fields_ident != "extra" {
        input
            .extra_fields_ident
            .span()
            .unwrap()
            .error("expected `extra`")
            .emit();

        return None;
    }

    if let Some(override_ident) = input.overrides_ident.clone() && override_ident != "overrides" {
        override_ident.span().unwrap().error("expected `overrides`").emit();

        return None;
    }

    if input.overrides_ident.is_some() {
        if input.equal4.is_some() && input.overrides_array.is_none() {
            input
                .equal4
                .span()
                .unwrap()
                .error("expected overrides array")
                .emit();

            return None;
        } else if input.equal4.is_none() {
            input
                .overrides_ident
                .span()
                .unwrap()
                .error("expected `=` and overrides array")
                .emit();

            return None;
        }
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

    let (mut fields_tokens, mut fields_assignments): (Vec<_>, Vec<_>) =
        match input.exclude_or_include_ident.to_string().as_str() {
            "exclude" => type_metadata
                .fields
                .iter()
                .filter_map(|field| {
                    if fields.contains(&field.name) {
                        None
                    } else {
                        let field_name = Ident::new(field.name.as_str(), Span::call_site());
                        let field_type = syn::parse_str::<Type>(
                            expand_fully_qualified_type_name(
                                field.ty.clone(),
                                input.overrides_array.clone(),
                            )
                            .as_str(),
                        )
                        .unwrap();

                        Some((
                            quote! {#field_name: #field_type},
                            quote! {#field_name: model.#field_name},
                        ))
                    }
                })
                .unzip(),
            "include" => type_metadata
                .fields
                .iter()
                .filter_map(|field| {
                    if fields.contains(&field.name) {
                        let field_name = Ident::new(field.name.as_str(), Span::call_site());
                        let field_type = syn::parse_str::<Type>(
                            expand_fully_qualified_type_name(
                                field.ty.clone(),
                                input.overrides_array.clone(),
                            )
                            .as_str(),
                        )
                        .unwrap();

                        Some((
                            quote! {pub #field_name: #field_type},
                            quote! {#field_name: model.#field_name},
                        ))
                    } else {
                        None
                    }
                })
                .unzip(),
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

    let (mut field_tokens_to_append, mut field_assignments_to_append) = type_metadata
        .fields
        .iter()
        .filter_map(|field| {
            if id_fields.contains(&field.name) {
                let field_name = Ident::new(field.name.as_str(), Span::call_site());

                let field_type = syn::parse_str::<Type>(
                    expand_fully_qualified_type_name(
                        field.ty.clone(),
                        input.overrides_array.clone(),
                    )
                    .as_str(),
                )
                .unwrap();

                Some((
                    quote! {pub #field_name: #field_type},
                    quote! {#field_name: model.#field_name},
                ))
            } else {
                None
            }
        })
        .unzip();

    fields_tokens.append(&mut field_tokens_to_append);
    fields_assignments.append(&mut field_assignments_to_append);

    let type_tokens = if id_fields.len() == 1 {
        let field = type_metadata
            .fields
            .iter()
            .find(|field| field.name == id_fields[0])
            .unwrap();

        syn::parse_str::<Type>(
            expand_fully_qualified_type_name(field.ty.clone(), input.overrides_array.clone())
                .as_str(),
        )
        .unwrap()
        .to_token_stream()
    } else {
        let vec = id_fields
            .iter()
            .map(|name| {
                type_metadata
                    .fields
                    .iter()
                    .find(|field| field.name == name.clone())
                    .unwrap()
            })
            .map(|field| {
                syn::parse_str::<Type>(
                    expand_fully_qualified_type_name(
                        field.ty.clone(),
                        input.overrides_array.clone(),
                    )
                    .as_str(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>();

        quote! {
            (#(#vec),*)
        }
    };

    let id_field_expr_tokens = if id_fields.len() == 1 {
        let ident = Ident::new(&id_fields[0], Span::call_site()).to_token_stream();
        quote::quote! {
            self.#ident
        }
    } else {
        let vec = id_fields
            .iter()
            .map(|name| {
                let ident = Ident::new(name, Span::call_site()).to_token_stream();
                quote! {
                    self.#ident
                }
            })
            .collect::<Vec<_>>();

        quote! {
            (#(#vec),*)
        }
    };

    let attrs = item_struct.attrs.clone();
    let from_type = syn::parse_str::<Type>(type_key.as_str()).unwrap();

    if input.extra_fields_array.elements.empty_or_trailing() {
        let from_type = from_type.clone();
        let attrs = attrs.clone();
        let fields_tokens = fields_tokens.clone();
        let type_tokens = type_tokens.clone();
        let id_field_expr_tokens = id_field_expr_tokens.clone();
        let fields_assignments = fields_assignments.clone();

        return Some(quote! {
            #(#attrs)*
            #item_struct_vis struct #item_struct_name {
                #(#fields_tokens),*
            }
            #[automatically_derived]
            impl hartex_discord_entitycache_core::traits::Entity for #item_struct_name {
                type Id = #type_tokens;
                fn id(&self) -> <Self as hartex_discord_entitycache_core::traits::Entity>::Id {
                    #id_field_expr_tokens
                }
            }
            impl From<#from_type> for #item_struct_name {
                fn from(model: #from_type) -> Self {
                    Self { #(#fields_assignments),* }
                }
            }
        });
    }

    let (extra_fields_tokens, extra_fields_assignment_tokens): (Vec<_>, Vec<_>) = input
        .extra_fields_array
        .elements
        .iter()
        .map(|element| {
            let ident = Ident::new(element.key.value().as_str(), Span::call_site());
            let type_token = syn::parse_str::<Type>(
                expand_fully_qualified_type_name(
                    element.value.value(),
                    input.overrides_array.clone(),
                )
                .as_str(),
            )
            .unwrap();

            (quote! {#ident: #type_token}, quote! {#ident})
        })
        .unzip();
    let extra_type_tokens = input
        .extra_fields_array
        .elements
        .iter()
        .map(|element| {
            let type_token = syn::parse_str::<Type>(
                expand_fully_qualified_type_name(
                    element.value.value(),
                    input.overrides_array.clone(),
                )
                .as_str(),
            )
            .unwrap();

            quote! {#type_token}
        })
        .collect::<Vec<_>>();

    Some(quote! {
        #(#attrs)*
        #item_struct_vis struct #item_struct_name {
            #(#fields_tokens),*
            #(#extra_fields_tokens),*
        }
        #[automatically_derived]
        impl hartex_discord_entitycache_core::traits::Entity for #item_struct_name {
            type Id = #type_tokens;
            fn id(&self) -> <Self as hartex_discord_entitycache_core::traits::Entity>::Id {
                #id_field_expr_tokens
            }
        }
        impl From<(#(#extra_type_tokens),* #from_type)> for #item_struct_name {
            fn from((#(#extra_fields_assignment_tokens),* model): (#(#extra_type_tokens),* #from_type)) -> Self {
                Self { #(#fields_assignments),* #(#extra_fields_assignment_tokens),* }
            }
        }
    })
}

fn expand_fully_qualified_type_name(
    mut to_expand: String,
    overrides_array: Option<KeyValueArray>,
) -> String {
    to_expand = to_expand.replace(' ', "");

    let open_angle_brackets = to_expand.find('<');
    let close_angle_brackets = to_expand.rfind('>');

    if open_angle_brackets.is_none() && close_angle_brackets.is_none() {
        if PRELUDE_AND_PRIMITIVES.contains(&to_expand.as_str()) {
            return to_expand;
        }

        if let Some(override_array) = overrides_array.clone()
            && let Some(element) = override_array.elements.iter()
            .find(|element| element.key.value() == to_expand)
        {
            return element.value.value();
        }

        let fully_qualified = if let Some(found) = metadata::ENUM_MAP.keys().find(|key| {
            let index = key.rfind(':').unwrap();
            key[index + 1..] == to_expand
        }) {
            found
        } else {
            let Some(found) = metadata::STRUCT_MAP.keys().find(|key| {
                let index = key.rfind(':').unwrap();
                key[index + 1..] == to_expand
            }) else {
                return to_expand;
            };

            found
        };

        return (*fully_qualified).to_string();
    }

    format!(
        "{}<{}>",
        expand_fully_qualified_type_name(
            to_expand[0..open_angle_brackets.unwrap()].to_string(),
            overrides_array.clone(),
        ),
        expand_fully_qualified_type_name(
            to_expand[open_angle_brackets.unwrap() + 1..close_angle_brackets.unwrap()].to_string(),
            overrides_array,
        )
    )
}
