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

use hartex_localization_core::Localizer;
use hartex_localization_core::LOCALIZATION_HOLDER;
use unic_langid::LanguageIdentifier;

use crate::localizable::Localizable;

impl Localizable for bool {
    fn localize(&self, locale: Option<LanguageIdentifier>) -> miette::Result<String> {
        let locale = locale.map_or(String::from("en-GB"), |locale| locale.to_string());
        let localizer = Localizer::new(&LOCALIZATION_HOLDER, &locale);

        Ok(if *self {
            localizer.general_boolean_true()?
        } else {
            localizer.general_boolean_false()?
        })
    }
}
