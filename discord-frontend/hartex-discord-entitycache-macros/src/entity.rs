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

use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;
use hartex_macro_utils::bail;
use hartex_macro_utils::impl_parse;
use pluralizer::pluralize;
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
use crate::reflect::Field;

const PRELUDE_AND_PRIMITIVES: [&str; 21] = [
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "&str",
    "bool", "char", "f32", "f64", "Option", "Box", "String", "Vec",
];

const VALID_ENTITIES: [(&str, &str); 4] = [
    ("GuildEntity", "crate::guild::GuildEntity"),
    ("MemberEntity", "crate::member::MemberEntity"),
    ("RoleEntity", "crate::role::RoleEntity"),
    ("UserEntity", "crate::user::UserEntity"),
];

impl_parse!(
#[allow(dead_code)]
#[allow(clippy::module_name_repetitions)]
pub struct EntityMacroInput where
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
    comma2: Token![,],
    overrides_ident: Ident,
    equal4: Token![=],
    overrides_array: KeyValueArray,
    comma4: Token![,],
    relates_idents: Ident,
    equal6: Token![=],
    relates_array: RelatesArray,
    comma6??: Token![,],
);

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

impl_parse!(
#[derive(Clone)]
struct KeyValueArrayElement where
    key: LitStr,
    colon: Token![:],
    value: LitStr,
);

#[derive(Clone)]
struct RelatesArray {
    #[allow(dead_code)]
    bracket_token: Bracket,
    #[allow(dead_code)]
    elements: Punctuated<RelatesArrayElement, Token![,]>,
}

impl Parse for RelatesArray {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let bracket_token = bracketed!(content in input);
        let mut elements = Punctuated::new();

        while !content.is_empty() {
            let first = content.parse::<RelatesArrayElement>()?;
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

impl_parse!(
#[derive(Clone)]
struct RelatesArrayElement where
    unique_or_multiple: Ident,
    name: LitStr,
    colon: Token![:],
    via: Ident,
    value: LitStr,
);

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::too_many_lines)]
pub fn implement_entity(input: &EntityMacroInput, item_struct: &ItemStruct) -> Option<TokenStream> {
    if input.from_ident != "from" {
        bail(&input.from_ident, "expected `from`")?;
    }

    if input.id_ident != "id" {
        bail(&input.from_ident, "expected `id`")?;
    }

    if input.extra_fields_ident != "extra" {
        bail(&input.extra_fields_ident, "expected `extra`")?;
    }

    if input.overrides_ident != "overrides" {
        bail(&input.overrides_ident, "expected `overrides`")?;
    }

    let type_key = input.from_lit_str.value();
    let rfind_index = type_key.rfind(':').unwrap();
    let end = &type_key[rfind_index..];

    let type_metadata = if let Some(key) =
        metadata::STRUCT_MAP.keys().find(|key| key.ends_with(end))
    {
        metadata::STRUCT_MAP.get(key).copied().unwrap()
    } else {
        (input.from_lit_str.span().unwrap())
            .error(format!("type `{type_key}` cannot be found"))
            .note(format!(
                "the type metadata generated was for twilight-model version {}",
                metadata::CRATE_VERSION
            ))
            .help("consider regenerating the metadata for a newer version if the type is recently added")
            .emit();
        return None;
    };

    let mut any_not_found = false;
    let fields = input.exclude_or_include_array.elems.iter().filter_map(|expr| {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                if type_metadata.fields.iter().any(|field| field.name == lit_str.value()) {
                    return Some(lit_str.value())
                }
                lit_str.span().unwrap()
                    .error(format!("field `{}` cannot be found in type `{type_key}`", lit_str.value()))
                    .note(format!(
                        "the type metadata generated was for twilight-model version {}",
                        metadata::CRATE_VERSION
                    ))
                    .help("consider regenerating the metadata for a newer version if the field is recently added")
                    .emit();
                any_not_found = true;
            }
            expr => expr.span().unwrap().warning("non-string expressions are ignored").emit(),
        }
        None
    });
    let fields: Vec<_> = fields.collect();

    let id_fields = input.id_array.elems.iter().filter_map(|expr| {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                if type_metadata.fields.iter().any(|field| field.name == lit_str.value()) {
                    return Some(lit_str.value());
                }

                if input.extra_fields_array.elements.iter().any(|element| element.key.value() == lit_str.value()) {
                    return Some(lit_str.value());
                }

                lit_str.span().unwrap()
                    .error(format!("field `{}` cannot be found in type `{type_key}`", lit_str.value()))
                    .note(format!(
                        "the type metadata generated was for twilight-model version {}",
                        metadata::CRATE_VERSION
                    ))
                    .help("consider regenerating the metadata for a newer version if the field is recently added")
                    .emit();
                any_not_found = true;
            }
            expr => bail(expr, "non-string expressions are ignored")?,
        }
        None
    });
    let id_fields = id_fields.collect::<Vec<_>>();

    if any_not_found {
        return None;
    }

    let item_struct_vis = item_struct.vis.clone();
    let item_struct_name = item_struct.ident.clone();

    let (mut fields_tokens, mut fields_assignments): (Vec<_>, Vec<_>) =
        match &*input.exclude_or_include_ident.to_string() {
            "exclude" => type_metadata
                .fields
                .iter()
                .filter_map(|field| {
                    if fields.contains(&field.name) {
                        None
                    } else {
                        let field_name = Ident::new(field.name.as_str(), Span::call_site());
                        let field_type = syn::parse_str::<Type>(&expand_fully_qualified_type_name(
                            &field.ty,
                            &input.overrides_array,
                        ))
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
                    fields.iter().find(|&x| x == &field.name).map(|_| {
                        let field_name = Ident::new(&field.name, Span::call_site());
                        let field_type = syn::parse_str::<Type>(&expand_fully_qualified_type_name(
                            &field.ty,
                            &input.overrides_array,
                        ))
                        .unwrap();

                        (
                            quote! {pub #field_name: #field_type},
                            quote! {#field_name: model.#field_name},
                        )
                    })
                })
                .unzip(),
            _ => bail(
                &input.exclude_or_include_ident,
                "expected `exclude` or `include`",
            )?,
        };

    let (mut field_tokens_to_append, mut field_assignments_to_append) = type_metadata
        .fields
        .iter()
        .filter_map(|field| {
            id_fields.iter().find(|&x| x == &field.name).map(|_| {
                let field_name = Ident::new(field.name.as_str(), Span::call_site());

                let field_type = syn::parse_str::<Type>(&expand_fully_qualified_type_name(
                    &field.ty,
                    &input.overrides_array,
                ))
                .unwrap();

                (
                    quote! {pub #field_name: #field_type},
                    quote! {#field_name: model.#field_name},
                )
            })
        })
        .unzip();

    fields_tokens.append(&mut field_tokens_to_append);
    fields_assignments.append(&mut field_assignments_to_append);

    let type_tokens = if let [first] = &id_fields[..] {
        let field = (type_metadata.fields.iter())
            .find(|field| &field.name == first)
            .unwrap();

        let s = syn::parse_str::<Type>(&expand_fully_qualified_type_name(
            &field.ty,
            &input.overrides_array,
        ));
        s.unwrap().to_token_stream()
    } else {
        let vec = (id_fields.iter())
            .map(|name| {
                (type_metadata.fields.iter().cloned())
                    .chain(
                        input
                            .extra_fields_array
                            .elements
                            .iter()
                            .map(|element| Field {
                                name: element.key.value(),
                                vis: "pub".to_string(),
                                ty: element.value.value(),
                            }),
                    )
                    .find(|field| &field.name == name)
                    .unwrap()
            })
            .map(|field| {
                syn::parse_str::<Type>(&expand_fully_qualified_type_name(
                    &field.ty,
                    &input.overrides_array,
                ))
                .unwrap()
            })
            .collect::<Vec<_>>();

        quote! {
            (#(#vec),*)
        }
    };

    let id_field_expr_tokens = if let [first] = &id_fields[..] {
        let ident = Ident::new(first, Span::call_site()).to_token_stream();
        quote! { self.#ident }
    } else {
        let vec = (id_fields.iter())
            .map(|name| {
                let ident = Ident::new(name, Span::call_site()).to_token_stream();
                quote! { self.#ident }
            })
            .collect::<Vec<_>>();

        quote! { (#(#vec),*) }
    };

    let attrs = &item_struct.attrs;
    let from_type = syn::parse_str::<Type>(&type_key).unwrap();

    let mut function_decls = Vec::new();
    for element in &input.relates_array.elements {
        if !["multiple", "unique"].contains(&element.unique_or_multiple.to_string().as_str()) {
            bail(
                &element.unique_or_multiple,
                "expected either `multiple` or `unique`",
            )?;
        }

        if element.via != "via" {
            bail(&element.via, "expected `via`")?;
        }

        let hashmap = VALID_ENTITIES.into_iter().collect::<HashMap<_, _>>();

        if !hashmap
            .keys()
            .any(|name| name == &element.name.value().as_str())
        {
            bail(&element.name, "unknown entity name")?;
        }

        let entity = element.name.value();
        let cased_entity = entity.to_case(Case::Snake);
        let first: &str = cased_entity.split("_").next().unwrap();

        let ret_type = syn::parse_str::<Type>(hashmap.get(entity.as_str()).unwrap()).unwrap();

        let function = if element.unique_or_multiple == "multiple" {
            let ident = Ident::new(&pluralize(first, 2, false), Span::call_site());

            quote! {
                async fn #ident() -> hartex_discord_entitycache_core::error::CacheResult<Vec<#ret_type>> {
                    todo!()
                }
            }
        } else if element.unique_or_multiple == "unique" {
            let ident = Ident::new(first, Span::call_site());

            quote! {
                async fn #ident() -> hartex_discord_entitycache_core::error::CacheResult<#ret_type> {
                    todo!()
                }
            }
        } else {
            unreachable!()
        };

        function_decls.push(quote! {#function});
    }

    if input.extra_fields_array.elements.is_empty() {
        return Some(quote! {
            use hartex_discord_utils::DATABASE_POOL;
            use tokio_postgres::GenericClient;

            #(#attrs)*
            #item_struct_vis struct #item_struct_name {
                #(#fields_tokens),*
            }
            impl #item_struct_name {
                #(#function_decls)*
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

    let fields = input.extra_fields_array.elements.iter().map(|element| {
        let ident = Ident::new(&element.key.value(), Span::call_site());
        let type_token = syn::parse_str::<Type>(&expand_fully_qualified_type_name(
            &element.value.value(),
            &input.overrides_array,
        ))
        .unwrap();

        (quote! {pub #ident: #type_token}, quote! {#ident})
    });
    let (extra_fields_tokens, extra_fields_assignment_tokens): (Vec<_>, Vec<_>) = fields.unzip();
    let extra_type_tokens = input.extra_fields_array.elements.iter().map(|element| {
        let type_token = syn::parse_str::<Type>(&expand_fully_qualified_type_name(
            &element.value.value(),
            &input.overrides_array,
        ))
        .unwrap();

        quote! {#type_token}
    });
    let extra_type_tokens: Vec<_> = extra_type_tokens.collect();

    Some(quote! {
        use hartex_discord_utils::DATABASE_POOL;
        use tokio_postgres::GenericClient;

        #(#attrs)*
        #item_struct_vis struct #item_struct_name {
            #(#fields_tokens),*,
            #(#extra_fields_tokens),*
        }
        impl #item_struct_name {
            #(#function_decls)*
        }
        #[automatically_derived]
        impl hartex_discord_entitycache_core::traits::Entity for #item_struct_name {
            type Id = #type_tokens;
            fn id(&self) -> <Self as hartex_discord_entitycache_core::traits::Entity>::Id {
                #id_field_expr_tokens
            }
        }
        impl From<(#(#extra_type_tokens),*, #from_type)> for #item_struct_name {
            fn from((#(#extra_fields_assignment_tokens),*, model): (#(#extra_type_tokens),*, #from_type)) -> Self {
                Self { #(#fields_assignments),*, #(#extra_fields_assignment_tokens),* }
            }
        }
    })
}

fn expand_fully_qualified_type_name(to_expand: &str, overrides_array: &KeyValueArray) -> String {
    let to_expand = to_expand.replace(' ', "");

    let open_angle_brackets = to_expand.find('<');
    let close_angle_brackets = to_expand.rfind('>');

    if open_angle_brackets.or(close_angle_brackets).is_some() {
        let (left, right) = open_angle_brackets.zip(close_angle_brackets).unwrap();
        return format!(
            "{}<{}>",
            expand_fully_qualified_type_name(&to_expand[0..left], overrides_array),
            expand_fully_qualified_type_name(&to_expand[left + 1..right], overrides_array)
        );
    }
    if PRELUDE_AND_PRIMITIVES.contains(&&*to_expand) {
        return to_expand;
    }

    if let Some(element) = overrides_array
        .elements
        .iter()
        .find(|elm| elm.key.value() == to_expand)
    {
        return element.value.value();
    }

    let finder = |key: &&&str| key[key.rfind(':').unwrap() + 1..] == to_expand;

    let fully_qualified = (metadata::ENUM_MAP.keys().find(finder))
        .or_else(|| metadata::STRUCT_MAP.keys().find(finder));

    fully_qualified.map_or(to_expand, ToString::to_string)
}
