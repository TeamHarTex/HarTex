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
use quote::format_ident;
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
        parameters
            .args_ident
            .span()
            .unwrap()
            .error(format!(
                "expected identifier `args`; found {}",
                parameters.args_ident
            ))
            .emit();

        return None;
    }

    let bundle = parameters.parameters.bundle_variable_name;
    let key = parameters.parameters.key_name_lit;
    let Lit::Str(_) = key else {
        key.span()
            .unwrap()
            .error("expected string literal")
            .emit();

        return None;
    };

    let errors = parameters.parameters.out_errors_ident;
    let value = parameters.parameters.out_value_ident;
    let ident_type = parameters.parameters.ident_type;
    let function_name = format_ident!("get_{}", ident_type);
    let format_pattern_param1 = if ident_type == "term" {
        quote::quote!(irrelevant.value())
    } else {
        quote::quote!(irrelevant.value().unwrap())
    };

    let mut args = quote::quote! {
        let mut args = hartex_localization_core::types::LocalizationArgs::new();
    };

    for arg in parameters.args {
        let Lit::Str(_) = arg.key_lit else {
            arg.key_lit.span()
                .unwrap()
                .error("expected string literal")
                .emit();

            return None;
        };

        if arg.to_ident != "to" {
            arg.to_ident
                .span()
                .unwrap()
                .error(format!("expected identifier `to`; found {}", arg.to_ident))
                .emit();

            return None;
        }

        let key_lit = arg.key_lit;
        let value_ident = arg.value_var_ident;
        args.extend(quote::quote! {
            args.set(#key_lit, #value_ident);
        });
    }

    Some(quote::quote! {
        let irrelevant = #bundle.#function_name(#key).unwrap();
        let mut #errors = Vec::new();
        #args
        let #value = #bundle.format_pattern(#format_pattern_param1, Some(&args), &mut errors);
        let #value = #value.trim();
    })
}
