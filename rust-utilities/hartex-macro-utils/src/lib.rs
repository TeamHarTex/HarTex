/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

//! # Macro Utilities

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro_diagnostic)]

use std::any::Any;

use quote::ToTokens;
use syn::spanned::Spanned;

/// Just bail.
pub fn bail<T: ToTokens, A: Any>(token: &T, msg: &str) -> Option<A> {
    token.span().unwrap().error(msg).emit();
    None
}

/// Expect a certain identifier.
#[macro_export]
macro_rules! expect {
    ($input:ident: $($attr:ident == $sth:literal);*) => {
        $(
            if $input.$attr != $sth {
                ::hartex_macro_utils::bail(&$input.$attr, concat!("expected `", $sth, "`"))?;
            }
        )*
    };
}

/// Implement parsing for AST objects.
#[macro_export]
macro_rules! impl_parse {
    (@type $t:ty) => { $t };
    (?@type $t:ty) => { Option<$t> };
    (@type $any:tt $t:ty) => { compile_error!("Expected `?` or nothing, found bad token"); };
    (@$input:ident?) => { $input.parse().ok() };
    (@$input:ident) => { $input.parse()? };
    ($(#[$x:meta])* $v:vis struct $name:ident where $($attr:ident$(?$qus:tt)?: $type:ty,)*) => {
        $(#[$x])* $v struct $name {
            $($attr: $crate::impl_parse!($($qus)?@type $type)),*
        }

        impl syn::parse::Parse for $name {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                Ok(Self {
                    $($attr: $crate::impl_parse!(@input$($qus)?),)*
                })
            }
        }
    };
}

/// Implement bracketed parsing for array AST objects.
#[macro_export]
macro_rules! impl_bracket_parse {
    (
        $(#[$m0:meta])* $v:vis struct $name:ident where
        $(#[$m1:meta])* $bracket_name:ident,
        $(#[$m2:meta])* $elements:ident => $elmtype:ty,
    ) => {
        $(#[$m0])* $v struct $name {
            $(#[$m1])* $bracket_name: ::syn::token::Bracket,
            $(#[$m2])* $elements: ::syn::punctuated::Punctuated<$elmtype, ::syn::Token![,]>,
        }

        impl ::syn::parse::Parse for $name {
            fn parse(input: ::syn::parse::ParseStream) -> ::syn::Result<Self> {
                let content;
                let bracket_token = ::syn::bracketed!(content in input);
                let mut elements = ::syn::punctuated::Punctuated::new();

                while !content.is_empty() {
                    let first = content.parse::<$elmtype>()?;
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
    }
}
