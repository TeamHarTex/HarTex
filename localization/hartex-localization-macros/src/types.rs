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
use syn::bracketed;
use syn::token;
use syn::Ident;
use syn::Lit;
use syn::punctuated::Punctuated;
use syn::Token;

pub struct Parameters {
    pub bundle_variable_name: Ident,
    pub dot: Token![.],
    pub key_name_lit: Lit,
    pub colon: Token![:],
    pub ident_type: Ident,
    pub comma1: Token![,],
    pub out_ident: Ident,
    pub brackets: token::Bracket,
    pub out_value_ident: Ident,
    pub comma2: Token![,],
    pub out_errors_ident: Ident,
}

impl Parse for Parameters {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            bundle_variable_name: input.parse()?,
            dot: input.parse()?,
            key_name_lit: input.parse()?,
            colon: input.parse()?,
            ident_type: input.parse()?,
            comma1: input.parse()?,
            out_ident: input.parse()?,
            brackets: bracketed!(content in input),
            out_value_ident: content.parse()?,
            comma2: content.parse()?,
            out_errors_ident: content.parse()?,
        })
    }
}

pub struct ParametersWithArgs {
    pub parameters: Parameters,
    pub args_ident: Ident,
    pub brackets: token::Bracket,
    pub args: Punctuated<Arg, Token![,]>,
}

impl Parse for ParametersWithArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            parameters: input.parse()?,
            args_ident: input.parse()?,
            brackets: bracketed!(content in input),
            args: Punctuated::<Arg, Token![,]>::parse_terminated(&content)?,
        })
    }
}

pub struct Arg {
    pub key_lit: Lit,
    pub to_ident: Ident,
    pub value_var_ident: Ident,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key_lit: input.parse()?,
            to_ident: input.parse()?,
            value_var_ident: input.parse()?,
        })
    }
}
