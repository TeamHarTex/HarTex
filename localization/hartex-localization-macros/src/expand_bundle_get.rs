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

use crate::types::Parameters;

pub fn expand_bundle_get(parameters: Parameters) -> Option<TokenStream2> {
    if parameters.out_ident != "out" {
        parameters
            .out_ident
            .span()
            .unwrap()
            .error(format!(
                "expected identifier `out`; found {}",
                parameters.out_ident
            ))
            .emit();

        return None;
    }

    let bundle = parameters.bundle_variable_name;
    let key = parameters.key_name_lit;
    let Lit::Str(_) = key else {
        key.span()
            .unwrap()
            .error("expected string literal")
            .emit();

        return None;
    };

    let errors = parameters.out_errors_ident;
    let value = parameters.out_value_ident;
    let ident_type = parameters.ident_type;
    let function_name = format_ident!("get_{}", ident_type);
    let format_pattern_param1 = if ident_type == "term" {
        quote::quote!(irrelevant.value())
    } else {
        quote::quote!(irrelevant.value().unwrap())
    };

    Some(quote::quote! {
        let irrelevant = #bundle.#function_name(#key).unwrap();
        let mut #errors = Vec::new();
        let #value = #bundle.format_pattern(#format_pattern_param1, None, &mut errors);
        let #value = #value.trim();
    })
}
