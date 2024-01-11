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

use rune::Any;
use rune::ContextError;
use rune::Module;

pub fn module() -> Result<Module, ContextError> {
    let mut module = Module::with_crate_item("hartexconf", ["colour"])?;

    module.ty::<Colour>()?;
    module.function_meta(Colour::raw)?;
    module.function_meta(Colour::rgb)?;

    Ok(module)
}

#[derive(Any)]
#[rune(item = ::hartexconf::colour)]
pub struct Colour(pub u32);

impl Colour {
    #[rune::function(path = Self::raw)]
    fn raw(colour: u32) -> Colour {
        Self(colour)
    }

    #[rune::function(path = Self::rgb)]
    fn rgb(r: u8, g: u8, b: u8) -> Colour {
        Self((r as u32) << 16 | (g as u32) << 8 | (b as u32))
    }
}
