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

use sqlparser::ast::ArrayElemTypeDef;
use sqlparser::ast::DataType;
use sqlparser::ast::TimezoneInfo;

pub(crate) fn sql_type_to_rust_type_token_stream(dt: DataType) -> Option<proc_macro2::TokenStream> {
    Some(match dt {
        DataType::Array(ArrayElemTypeDef::SquareBracket(deref!(ref dt_inner), _)) => {
            let Some(ts) = sql_type_to_rust_type_token_stream(dt_inner.clone()) else {
                return None;
            };

            quote::quote! {Vec<#ts>}
        }
        DataType::BigInt(_) => quote::quote! {i64},
        DataType::Boolean => quote::quote! {bool},
        DataType::Char(_)
        | DataType::Character(_)
        | DataType::CharacterVarying(_)
        | DataType::Varchar(_)
        | DataType::Text => quote::quote! {String},
        DataType::Real => quote::quote! {f32},
        DataType::SmallInt(_) => quote::quote! {i16},
        DataType::Integer(_) => quote::quote! {i32},
        DataType::Time(_, tz)
            if matches!(tz, TimezoneInfo::None | TimezoneInfo::WithoutTimeZone) =>
        {
            quote::quote! {time::Time}
        }
        DataType::Timestamp(_, tz) => match tz {
            TimezoneInfo::None | TimezoneInfo::WithoutTimeZone => {
                quote::quote! {time::PrimitiveDateTime}
            }
            TimezoneInfo::WithTimeZone | TimezoneInfo::Tz => quote::quote! {time::OffsetDateTime},
        },
        _ => return None,
    })
}

pub(crate) fn sql_type_to_rust_reftype_token_stream(
    dt: DataType,
) -> Option<proc_macro2::TokenStream> {
    Some(match dt {
        DataType::Array(ArrayElemTypeDef::SquareBracket(deref!(ref dt_inner), _)) => {
            let Some(ts) = sql_type_to_rust_type_token_stream(dt_inner.clone()) else {
                return None;
            };

            quote::quote! {&[#ts]}
        }
        DataType::BigInt(_) => quote::quote! {i64},
        DataType::Boolean => quote::quote! {bool},
        DataType::Char(_)
        | DataType::Character(_)
        | DataType::CharacterVarying(_)
        | DataType::Varchar(_)
        | DataType::Text => quote::quote! {&str},
        DataType::Real => quote::quote! {f32},
        DataType::SmallInt(_) => quote::quote! {i16},
        DataType::Integer(_) => quote::quote! {i32},
        DataType::Time(_, tz)
            if matches!(tz, TimezoneInfo::None | TimezoneInfo::WithoutTimeZone) =>
        {
            quote::quote! {time::Time}
        }
        DataType::Timestamp(_, tz) => match tz {
            TimezoneInfo::None | TimezoneInfo::WithoutTimeZone => {
                quote::quote! {time::PrimitiveDateTime}
            }
            TimezoneInfo::WithTimeZone | TimezoneInfo::Tz => quote::quote! {time::OffsetDateTime},
        },
        _ => return None,
    })
}
