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

#![feature(proc_macro_diagnostic)]

use quote::ToTokens;
use syn::spanned::Spanned;

pub fn bail<T: ToTokens, A: std::any::Any>(token: &T, msg: &str) -> Option<A> {
    token.span().unwrap().error(msg).emit();
    None
}

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
