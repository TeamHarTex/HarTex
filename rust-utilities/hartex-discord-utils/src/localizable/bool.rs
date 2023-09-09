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

use hartex_localization_core::create_bundle;
use hartex_localization_core::handle_errors;
use hartex_localization_macros::bundle_get;
use unic_langid::LanguageIdentifier;

use crate::localizable::Localizable;

impl Localizable for bool {
    fn localize(&self, locale: Option<LanguageIdentifier>) -> miette::Result<String> {
        let bundle = create_bundle(
            locale,
            &["discord-frontend"],
        )?;

        if *self {
            bundle_get!(bundle."boolean-true": message, out [boolean_true, errors]);
            handle_errors(errors)?;

            Ok(boolean_true.to_owned())
        } else {
            bundle_get!(bundle."boolean-false": message, out [boolean_false, errors]);
            handle_errors(errors)?;

            Ok(boolean_false.to_owned())
        }
    }
}
