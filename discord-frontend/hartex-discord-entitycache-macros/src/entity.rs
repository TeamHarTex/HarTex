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

use proc_macro::Diagnostic;
use proc_macro2::Delimiter;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::Data;
use syn::DataEnum;
use syn::DataStruct;
use syn::DataUnion;
use syn::DeriveInput;
use syn::Visibility;

#[allow(clippy::too_many_lines)]
pub fn expand_entity_derivation(input: &mut DeriveInput) -> Option<TokenStream2> {
    // check if item is public
    match input.vis.clone() {
        Visibility::Public(_) => {}
        visibility => {
            visibility.span().unwrap()
                .error("trait can only be derived on pub items")
                .emit();

            return None;
        }
    }

    // check if item is a struct
    match input.data.clone() {
        Data::Struct(_) => {}
        Data::Enum(DataEnum { enum_token, .. }) => {
            enum_token.span().unwrap()
                .error("trait can only be derived on structs")
                .emit();

            return None;
        }
        Data::Union(DataUnion { union_token, .. }) => {
            union_token.span().unwrap()
                .error("trait can only be derived on structs")
                .emit();

            return None;
        }
    }

    let Data::Struct(DataStruct { fields, .. }) = input.data.clone() else {
        unreachable!()
    };

    let iter = fields.into_iter();
    let mut map = iter.clone().map(|field| field.attrs);
    if !map.any(|attrs| !attrs.is_empty()) {
        Span::call_site().unwrap()
            .error("no field with `entity(..)` attribute")
            .emit();

        return None;
    }

    let attrs_iter = iter.map(|field| (field.clone(), field.attrs));
    let mut field_names = Vec::new();
    let mut id_tys = Vec::new();
    for (field, attrs) in attrs_iter {
        let mut invalid_attrs = attrs.clone();

        // look for non-entity attributes
        invalid_attrs.retain(|attr| !attr.path.is_ident("entity"));

        if !invalid_attrs.is_empty() {
            let _ = invalid_attrs
                .into_iter()
                .map(|attr| attr.path.span().unwrap())
                .map(|span| span.error("expected `entity` attribute"))
                .map(Diagnostic::emit);

            return None;
        }

        // all attributes are entity attributes
        for attr in attrs {
            let mut tree_iter = attr.tokens.into_iter();

            let tree = tree_iter.next();
            if tree.is_none() {
                attr.path.span().unwrap()
                    .error("unexpected end of attribute")
                    .emit();

                return None;
            }

            let TokenTree::Group(group) = tree.clone().unwrap() else {
                tree.span().unwrap()
                    .error("expected token group")
                    .emit();

                return None;
            };

            if group.delimiter() != Delimiter::Parenthesis {
                group.span().unwrap()
                    .error("expected parenthesized token group")
                    .emit();

                return None;
            }

            let mut group_iter = group.stream().into_iter();

            let tree = group_iter.next();
            if tree.is_none() {
                group.span().unwrap()
                    .error("unexpected end of attribute")
                    .emit();

                return None;
            }

            let TokenTree::Ident(ident) = tree.clone().unwrap() else {
                tree.span().unwrap()
                    .error("expected identifier")
                    .emit();

                return None;
            };

            if ident != "id" {
                ident.span().unwrap()
                    .error(format!("expected `id`; found `{ident}`"))
                    .emit();
                
                return None;
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
        #expanded
    })
}
