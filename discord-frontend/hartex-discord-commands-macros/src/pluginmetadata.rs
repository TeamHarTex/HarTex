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

use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::ItemStruct;
use syn::Lit;
use syn::Token;

/// Represents input to the `metadata` derive macro.
#[allow(dead_code)]
pub struct PluginMetadataMacroInput {
    pub(self) name_ident: Ident,
    pub(self) equal1: Token![=],
    pub(self) name_lit: Lit,
    pub(self) comma1: Option<Token![,]>,
}

impl Parse for PluginMetadataMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name_ident: input.parse()?,
            equal1: input.parse()?,
            name_lit: input.parse()?,
            comma1: input.parse().ok(),
        })
    }
}

/// Returns the token stream for generating the `PluginMetadata` trait implementation
#[allow(clippy::too_many_lines)]
pub fn implement_metadata(
    parameters: &PluginMetadataMacroInput,
    struct_item: &ItemStruct,
) -> Option<TokenStream2> {
    if parameters.name_ident != "name" {
        parameters
            .name_ident
            .span()
            .unwrap()
            .error("expected `name`")
            .emit();

        return None;
    }

    let core_use = quote::quote! {
        extern crate hartex_discord_commands_core as _commands_core;
    };
    let ident = struct_item.ident.clone();
    let lit = parameters.name_lit.clone();
    Some(quote::quote! {
        #core_use

        #struct_item

        #[automatically_derived]
        impl _commands_core::traits::PluginMetadata for #ident {
            pub fn name(&self) -> String {
                String::from(#lit)
            }
        }
    })
}
