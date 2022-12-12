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

use hartex_macro_utils::traits::SpanUtils;
use proc_macro2::{Delimiter, Span, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Visibility};

#[allow(clippy::too_many_lines)]
pub fn expand_entity_derivation(input: &mut DeriveInput) -> Result<TokenStream2, Vec<Error>> {
    // check if item is public
    match input.vis.clone() {
        Visibility::Public(_) => {}
        visibility => {
            return Err(vec![visibility
                .span()
                .error("trait can only be derived on pub items")]);
        }
    }

    // check if item is a struct
    match input.data.clone() {
        Data::Struct(_) => {}
        Data::Enum(DataEnum { enum_token, .. }) => {
            return Err(vec![enum_token
                .span()
                .error("trait can only be derived on structs")]);
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return Err(vec![union_token
                .span()
                .error("trait can only be derived on structs")]);
        }
    }

    let Data::Struct(DataStruct { fields, .. }) = input.data.clone() else {
        unreachable!()
    };

    let iter = fields.into_iter();
    let mut map = iter.clone().map(|field| field.attrs);
    if !map.any(|attrs| !attrs.is_empty()) {
        return Err(vec![
            Span::call_site().error("no field with `entity(..)` attribute")
        ]);
    }

    let attrs_iter = iter.map(|field| (field.clone(), field.attrs));
    let mut field_names = Vec::new();
    let mut id_tys = Vec::new();
    for (field, attrs) in attrs_iter {
        let mut invalid_attrs = attrs.clone();

        // look for non-entity attributes
        invalid_attrs.retain(|attr| !attr.path.is_ident("entity"));

        if !invalid_attrs.is_empty() {
            return Err(invalid_attrs
                .into_iter()
                .map(|attr| attr.path.span().error("expected `entity` attribute"))
                .collect::<Vec<_>>());
        }

        // all attributes are entity attributes
        for attr in attrs {
            let mut tree_iter = attr.tokens.into_iter();

            let tree = tree_iter.next();
            if tree.is_none() {
                return Err(vec![attr.path.span().error("unexpected end of attribute")]);
            }

            let TokenTree::Group(group) = tree.clone().unwrap() else {
                return Err(vec![tree.span().error("expected token group")]);
            };

            if group.delimiter() != Delimiter::Parenthesis {
                return Err(vec![group
                    .span()
                    .error("expected parenthesized token group")]);
            }

            let mut group_iter = group.stream().into_iter();

            let tree = group_iter.next();
            if tree.is_none() {
                return Err(vec![group.span().error("unexpected end of attribute")]);
            }

            let TokenTree::Ident(ident) = tree.clone().unwrap() else {
                return Err(vec![tree.span().error("expected identifier")]);
            };

            if ident != "id" {
                return Err(vec![ident
                    .span()
                    .error(format!("expected `id`; found `{ident}`"))]);
            }

            if field.ident.is_none() {
                return Err(vec![field
                    .ty
                    .span()
                    .error("cannot apply id attribute on tuple structs")]);
            }

            field_names.push(field.ident.clone().unwrap());
            id_tys.push(field.ty.clone());
        }
    }

    let type_tokens = if id_tys.len() == 1 {
        let ty = id_tys[0].to_token_stream();
        quote::quote! {
            #ty
        }
    } else {
        quote::quote! {
            ( #(#id_tys ,)* )
        }
    };

    let field_tokens = if field_names.len() == 1 {
        let ident = field_names[0].to_token_stream();
        quote::quote! {
            self.#ident
        }
    } else {
        quote::quote! {
            ( #(self.#field_names ,)* )
        }
    };

    let core_use = quote::quote! {
        extern crate hartex_discord_entitycache_core as _entitycache_core;
    };
    let dummy_const = quote::format_ident!("_");
    let ident = input.ident.clone();
    let expanded = quote::quote! {
        #core_use

        #[automatically_derived]
        impl _entitycache_core::traits::Entity for #ident {
            type Id = #type_tokens;

            fn id(&self) -> <Self as _entitycache_core::traits::Entity>::Id {
                #field_tokens
            }
        }
    };

    Ok(quote::quote! {
        const #dummy_const: () = {
            #expanded
        };
    })
}
