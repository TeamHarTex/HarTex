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

use proc_macro2::TokenTree;
use proc_macro2::TokenStream as TokenStream2;
use syn::Lit;

use crate::types::ParametersWithArgs;

pub fn expand_bundle_get_args(parameters: ParametersWithArgs) -> Option<TokenStream2> {
    if parameters.parameters.out_ident != "out" {
        parameters
            .parameters
            .out_ident
            .span()
            .unwrap()
            .error(format!(
                "expected identifier `out`; found {}",
                parameters.parameters.out_ident
            ))
            .emit();

        return None;
    }

    if parameters.args_ident != "args" {
        parameters.args_ident.span()
            .unwrap()
            .error(format!("expected identifier `args`; found {}", parameters.args_ident))
            .emit();

        return None;
    }

    for arg in parameters.args {
        let Lit::Str(_) = arg.key_lit else {
            arg.key_lit.span()
                .unwrap()
                .error("expected string literal")
                .emit();

            return None;
        };

        if arg.to_ident != "to" {
            arg.to_ident.span()
                .unwrap()
                .error(format!("expected identifier `to`; found {}", arg.to_ident))
                .emit();

            return None;
        }
    }

    todo!()
}
