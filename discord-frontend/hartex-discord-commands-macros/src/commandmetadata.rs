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

use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::spanned::Spanned;
use syn::{AttrStyle, Data, DataEnum, DataUnion, DeriveInput, Error, Visibility};

pub fn expand_command_metadata_derivation(
    input: &mut DeriveInput,
) -> Result<TokenStream2, Vec<Error>> {
    // check if item is public
    match input.vis.clone() {
        Visibility::Public(_) => {}
        visibility => {
            return Err(vec![Error::new(
                visibility.span(),
                "trait can only be derived on pub items",
            )]);
        }
    }

    // check if item is a struct
    match input.data.clone() {
        Data::Struct(_) => {}
        Data::Enum(DataEnum { enum_token, .. }) => {
            return Err(vec![Error::new(
                enum_token.span(),
                "trait can only be derived on structs",
            )]);
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return Err(vec![Error::new(
                union_token.span(),
                "trait can only be derived on structs",
            )]);
        }
    }

    // split attribute vector into two
    let mut wrong_paths = input.attrs.clone();
    let correct_attrs = wrong_paths
        .drain_filter(|attr| attr.style == AttrStyle::Outer && attr.path.is_ident("metadata"))
        .collect::<Vec<_>>();

    if !wrong_paths.is_empty() {
        return Err(wrong_paths
            .into_iter()
            .map(|attr| attr.path.span())
            .map(|span| Error::new(span, "expected `metadata` attribute"))
            .collect());
    }

    if correct_attrs.is_empty() {
        return Err(vec![Error::new(
            Span::call_site(),
            "expected `metadata` attributes after derive",
        )]);
    }

    for attr in correct_attrs {
        if attr.tokens.is_empty() {
            return Err(vec![Error::new(attr.path.span(), "unexpected end of attribute")]);
        }
    }

    /*let mut ret = TokenStream::new();
    let Some(stream) = crate::internal::derive::DeriveStream::parse(tokens) else {
        return ret;
    };

    let mut functions = TokenStream::new();
    stream.attributes.into_iter().for_each(|attribute| {
        let expanded = match attribute {
            DeriveAttribute::Description(description) => proc_macro::quote! {
                fn description(&self) -> String {
                    String::from($description)
                }
            },
            DeriveAttribute::InteractionOnly(interaction_only) => proc_macro::quote! {
                fn interaction_only(&self) -> bool {
                    $interaction_only
                }
            },
            DeriveAttribute::Name(name) => proc_macro::quote! {
                fn name(&self) -> String {
                    String::from($name)
                }
            },
            DeriveAttribute::CommandType(command_type) => proc_macro::quote! {
                fn command_type(&self) -> u8 {
                    $command_type
                }
            },
        };

        functions.extend(expanded);
    });

    let Some(ident) = stream.identifier else {
        return ret;
    };
    let expanded = proc_macro::quote! {
        impl hartex_discord_commands_core::CommandMetadata for $ident {
            $functions
        }
    };
    ret.extend(expanded);

    ret*/

    Ok(TokenStream2::new())
}
