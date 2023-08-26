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

use proc_macro2::TokenStream as TokenStream2;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::ItemStruct;
use syn::Lit;
use syn::Ident;
use syn::Token;

#[allow(dead_code)]
pub struct MetadataMacroInput {
    pub(self) command_type_ident: Ident,
    pub(self) equal1: Token![=],
    pub(self) command_type_lit: Lit,
    pub(self) comma1: Token![,],
    pub(self) interaction_only_ident: Ident,
    pub(self) equal2: Token![=],
    pub(self) interaction_only_lit: Lit,
    pub(self) comma2: Token![,],
    pub(self) name_ident: Ident,
    pub(self) equal3: Token![=],
    pub(self) name_lit: Lit,
    pub(self) comma3: Option<Token![,]>,
}

impl Parse for MetadataMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            command_type_ident: input.parse()?,
            equal1: input.parse()?,
            command_type_lit: input.parse()?,
            comma1: input.parse()?,
            interaction_only_ident: input.parse()?,
            equal2: input.parse()?,
            interaction_only_lit: input.parse()?,
            comma2: input.parse()?,
            name_ident: input.parse()?,
            equal3: input.parse()?,
            name_lit: input.parse()?,
            comma3: input.parse().ok()
        })
    }
}

#[allow(clippy::too_many_lines)]
pub fn implement_metadata(parameters: &MetadataMacroInput, struct_item: &ItemStruct) -> Option<TokenStream2> {
    if parameters.command_type_ident != "command_type" {
        parameters
            .command_type_ident
            .span()
            .unwrap()
            .error("expected `command_type`")
            .emit();

        return None;
    }

    if parameters.interaction_only_ident != "interaction_only" {
        parameters
            .interaction_only_ident
            .span()
            .unwrap()
            .error("expected `interaction_only`")
            .emit();

        return None;
    }

    if parameters.name_ident != "name" {
        parameters
            .name_ident
            .span()
            .unwrap()
            .error("expected `name`")
            .emit();

        return None;
    }

    let mut functions = TokenStream2::new();

    // command_type = ?
    let Lit::Int(command_type_literal) = parameters.command_type_lit.clone() else {
        parameters
            .command_type_lit
            .span()
            .unwrap()
            .error("expected integer literal")
            .emit();

        return None;
    };
    let Ok(command_type) = command_type_literal.base10_digits().parse::<u8>() else {
        unreachable!()
    };

    if !(1..=3).contains(&command_type) {
        parameters
            .command_type_lit
            .span()
            .unwrap()
            .error("invalid command type")
            .emit();

        return None;
    }

    let expanded = quote::quote! {
        fn command_type(&self) -> u8 {
            #command_type_literal
        }
    };
    functions.extend(expanded);

    // interaction_only = ?
    let Lit::Bool(interaction_only_literal) = parameters.interaction_only_lit.clone() else {
        parameters
            .interaction_only_lit
            .span()
            .unwrap()
            .error("expected boolean")
            .emit();

        return None;
    };

    let expanded = quote::quote! {
        fn interaction_only(&self) -> bool {
            #interaction_only_literal
        }
    };
    functions.extend(expanded);

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
