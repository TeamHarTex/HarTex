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

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::FluentResource;
use intl_memoizer::concurrent::IntlLangMemoizer as ConcurrentIntlLangMemoizer;
use miette::IntoDiagnostic;
use unic_langid::LanguageIdentifier;

mod env;

pub struct LocalizationBundleHolder {
    bundles: HashMap<String, LocalizationBundle>
}

impl LocalizationBundleHolder {
    pub fn load_localizations() -> miette::Result<Self> {
        let base_path = env::base_path();
        let mut bundles = HashMap::new();

        let dir_handle = fs::read_dir(base_path.clone()).into_diagnostic()?;

        for result in dir_handle {
            let entry_handle = result.into_diagnostic()?;

            let meta = entry_handle.file_type().into_diagnostic()?;
            if !meta.is_dir() {
                continue;
            }

            let file_name = entry_handle.file_name();
            let lang_name = file_name.to_string_lossy();

            let Ok(lang_ident) = lang_name.parse::<LanguageIdentifier>() else {
                continue;
            };

            let bundle = load_bundle(base_path.clone(), lang_ident)?;
            bundles.insert(lang_name.to_string(), bundle);
        }

        Ok(Self {
            bundles,
        })
    }

    pub fn get_bundle(&self, lang: &str) -> Option<&LocalizationBundle> {
        self.bundles.get(lang)
    }
}

fn load_bundle(mut base_path: PathBuf, lang_ident: LanguageIdentifier) -> miette::Result<LocalizationBundle> {
    let lang_name = lang_ident.to_string();
    base_path.push(&lang_name);

    let mut bundle = LocalizationBundle::new_concurrent(vec![lang_ident]);

    for resource in load_resources(base_path)? {
        bundle.add_resource_overriding(Arc::new(resource));
    }

    Ok(bundle)
}

fn load_resources(path: PathBuf) -> miette::Result<Vec<FluentResource>> {
    let mut loaded = Vec::new();

    let dir_handle = fs::read_dir(path).into_diagnostic()?;

    for result in dir_handle {
        let entry_handle = result.into_diagnostic()?;

        let meta = entry_handle.file_type().into_diagnostic()?;
        if meta.is_dir() {
            continue;
        }

        let file_name = entry_handle.file_name();
        let name = file_name.to_string_lossy();
        if !name.ends_with(".ftl") {
            continue;
        }

        let content = fs::read_to_string(entry_handle.path()).into_diagnostic()?;
        // todo: handle errors better here
        let resource = FluentResource::try_new(content).unwrap();

        loaded.push(resource);
    }

    Ok(loaded)
}

type LocalizationBundle = FluentBundle<Arc<FluentResource>, ConcurrentIntlLangMemoizer>;
