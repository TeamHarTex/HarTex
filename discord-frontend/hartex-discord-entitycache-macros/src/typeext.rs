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

use syn::GenericArgument;
use syn::PathArguments;
use syn::Type;

pub trait TypeExt {
    fn is_enum(&self, name: &str) -> bool;

    fn is_id(&self) -> bool;

    fn is_option_of(&self, option_of: &str) -> bool;

    fn is_vec_of(&self, vec_of: &str) -> bool;
}

impl TypeExt for Type {
    fn is_enum(&self, name: &str) -> bool {
        let Type::Path(path) = self else {
            return false;
        };

        let last = path.path.segments.last().unwrap();
        last.ident == name && last.arguments.is_none()
    }

    fn is_id(&self) -> bool {
        let Type::Path(path) = self else {
            return false;
        };

        let last = path.path.segments.last().unwrap();
        last.ident == "Id"
    }

    fn is_option_of(&self, option_of: &str) -> bool {
        let Type::Path(path) = self else {
            return false;
        };

        let last = path.path.segments.last().unwrap();

        if last.ident == "Option"
            && let PathArguments::AngleBracketed(angle_bracketed) = &last.arguments
            && let Some(GenericArgument::Type(Type::Path(path))) = angle_bracketed.args.first()
        {
            let last = path.path.segments.last().unwrap();
            return last.ident == option_of;
        }
        false
    }

    fn is_vec_of(&self, vec_of: &str) -> bool {
        let Type::Path(path) = self else {
            return false;
        };

        let last = path.path.segments.last().unwrap();
        if last.ident == "Vec"
            && let PathArguments::AngleBracketed(angle_bracketed) = &last.arguments
            && let Some(GenericArgument::Type(Type::Path(path))) = angle_bracketed.args.first()
        {
            let last = path.path.segments.last().unwrap();
            return last.ident == vec_of;
        }
        false
    }
}
