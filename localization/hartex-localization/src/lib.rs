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
#![feature(let_chains)]

use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use fluent_bundle::FluentResource;
use hartex_eyre::eyre::Report;
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

pub mod types;

pub fn create_bundle(
    requested: Option<LanguageIdentifier>,
    modules: Vec<&str>,
) -> hartex_eyre::Result<types::LocalizationBundle> {
    let fallback = langid!("en-US");
    let locale = requested.clone().unwrap_or(fallback);
    let mut bundle = types::LocalizationBundle::new(vec![locale.clone()]);

    let mut localizations_root = PathBuf::from("../localization/locales");
    localizations_root.push(locale.to_string());
    modules
        .iter()
        .for_each(|module| localizations_root.push(module));

    if !localizations_root.try_exists()? {
        return Err(Report::msg(format!(
            "localization root not found: {}",
            localizations_root.to_string_lossy()
        )));
    }

    if !localizations_root.is_dir() {
        return Err(Report::msg(format!(
            "localization root is not a directory: {}",
            localizations_root.to_string_lossy()
        )));
    }

    for result in localizations_root.read_dir()? {
        let entry = result?;
        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("ftl") {
            continue;
        }

        let resource_string = fs::read_to_string(path)?;
        let resource = FluentResource::try_new(resource_string)
            .map_err(|(_, errors)| errors.last().unwrap().clone())?;
        bundle
            .add_resource(resource)
            .map_err(|errors| errors.last().unwrap().clone())?;
    }

    Ok(bundle)
}
