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

use hartex_macro_utils::traits::SpanUtils;
use proc_macro2::{Span, TokenStream as TokenStream2};
use syn::spanned::Spanned;
use syn::{Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Visibility};

pub fn expand_entity_derivation(input: &mut DeriveInput) -> Result<TokenStream2, Vec<Error>> {
    // check if item is public
    match input.vis.clone() {
        Visibility::Public(_) => {}
        visibility => {
            return Err(vec![visibility
                .span()
                .error("trait can only be derived on pub items")]);
        }
    }

    // check if item is a struct
    match input.data.clone() {
        Data::Struct(_) => {}
        Data::Enum(DataEnum { enum_token, .. }) => {
            return Err(vec![enum_token
                .span()
                .error("trait can only be derived on structs")]);
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return Err(vec![union_token
                .span()
                .error("trait can only be derived on structs")]);
        }
    }

    let Data::Struct(DataStruct { fields, .. }) = input.data.clone() else {
        unreachable!()
    };

    let iter = fields.into_iter();
    let mut map = iter.clone().map(|field| field.attrs);
    if !map.any(|attrs| !attrs.is_empty()) {
        return Err(vec![
            Span::call_site().error("no field with `entity(..)` attribute")
        ]);
    }

    let attrs_iter = iter.map(|field| (field.clone(), field.attrs));
    for (_, attrs) in attrs_iter {
        let mut invalid_attrs = attrs.clone();

        // look for non-entity attributes
        invalid_attrs.retain(|attr| !attr.path.is_ident("entity"));

        if !invalid_attrs.is_empty() {
            return Err(invalid_attrs
                .into_iter()
                .map(|attr| attr.path.span().error("expected `entity` attribute"))
                .collect::<Vec<_>>());
        }

        // all attributes are entity attributes
    }

    Ok(TokenStream2::new())
}
