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

#![allow(incomplete_features)]
#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use unic_langid::langid;
use unic_langid::LanguageIdentifier;

pub mod types;

pub fn get_bundle(requested: Option<LanguageIdentifier>) -> types::LocalizationBundle {
    let locale = requested.unwrap_or(langid!("en-GB"));
    let _ = types::LocalizationBundle::new(vec![locale]);

    todo!()
}
