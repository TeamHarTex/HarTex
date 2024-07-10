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

use proc_macro2::TokenStream as TokenStream2;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::Expr;
use syn::Ident;
use syn::ItemStruct;
use syn::Lit;
use syn::Token;

/// Represents input to the `metadata` derive macro.
#[allow(dead_code)]
pub struct CommandMetadataMacroInput {
    pub(self) name_ident: Ident,
    pub(self) equal3: Token![=],
    pub(self) name_lit: Lit,
    pub(self) comma1: Token![,],
    pub(self) plugin_ident: Ident,
    pub(self) equal1: Token![=],
    pub(self) plugin_actual_ident: Ident,
    pub(self) comma3: Option<Token![,]>,
    pub(self) minimum_permission_level_ident: Option<Ident>,
    pub(self) equal_2: Option<Token![=]>,
    pub(self) minimum_permission_level: Option<Expr>,
    pub(self) comma4: Option<Token![,]>,
}

impl Parse for CommandMetadataMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut result = Self {
            name_ident: input.parse()?,
            equal3: input.parse()?,
            name_lit: input.parse()?,
            comma1: input.parse()?,
            plugin_ident: input.parse()?,
            equal1: input.parse()?,
            plugin_actual_ident: input.parse()?,
            comma3: None,
            minimum_permission_level_ident: None,
            equal_2: None,
            minimum_permission_level: None,
            comma4: None,
        };

        let Some(comma_3) = input.parse().ok() else {
            return Ok(result);
        };

        result.comma3.replace(comma_3);
        result.minimum_permission_level_ident = input.parse().ok();
        result.equal_2 = input.parse().ok();
        result.comma4 = input.parse().ok();

        Ok(result)
    }
}

/// Returns the token stream for generating the `CommandMetadata` trait implementation
#[allow(clippy::too_many_lines)]
pub fn implement_metadata(
    parameters: &CommandMetadataMacroInput,
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

    if parameters.plugin_ident != "plugin" {
        parameters
            .plugin_ident
            .span()
            .unwrap()
            .error("expected `plugin`")
            .emit();

        return None;
    }

    let mut functions = TokenStream2::new();

    // name = ?
    let Lit::Str(name) = parameters.name_lit.clone() else {
        parameters
            .name_lit
            .span()
            .unwrap()
            .error("expected string")
            .emit();

        return None;
    };
    let expanded = quote::quote! {
        fn name(&self) -> String {
            String::from(#name)
        }
    };
    functions.extend(expanded);

    // plugin = ?
    let plugin_ident = parameters.plugin_actual_ident.clone();
    let expanded = quote::quote! {
        fn plugin(&self) -> Box<dyn _commands_core::traits::Plugin + Send + Sync> {
            Box::new(#plugin_ident)
        }
    };
    functions.extend(expanded);

    // minimum_permission_level = ?
    if let Some(minimum_permission_level_ident) = parameters.minimum_permission_level_ident.clone()
        && minimum_permission_level_ident == "required_permissions"
    {
        let Some(_) = parameters.equal_2 else {
            parameters
                .equal_2
                .span()
                .unwrap()
                .error("expected equal")
                .emit();

            return None;
        };

        let Some(Expr::Binary(expr)) = parameters.minimum_permission_level.clone() else {
            parameters
                .minimum_permission_level
                .span()
                .unwrap()
                .error("expected expression")
                .emit();

            return None;
        };

        let expanded = quote::quote! {
            fn minimum_permission_level(&self) -> u8 {
                0
            }
        };
        functions.extend(expanded);
    }

    let core_use = quote::quote! {
        extern crate hartex_discord_commands_core as _commands_core;
    };
    let ident = struct_item.ident.clone();
    let expanded = quote::quote! {
        #core_use

        #struct_item

        #[automatically_derived]
        impl _commands_core::traits::CommandMetadata for #ident {
            #functions
        }
    };

    Some(quote::quote! {
       #expanded
    })
}
