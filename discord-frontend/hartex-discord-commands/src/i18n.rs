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

use std::sync::LazyLock;
use i18n_embed::{
    I18nEmbedError,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use rust_embed::RustEmbed;

pub static LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| fluent_language_loader!());

#[derive(RustEmbed)]
#[folder = "i18n"]
pub struct Localizations;

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn load_localizations() -> Result<(), I18nEmbedError> {
    i18n_embed::select(&*LOADER, &Localizations, &["en-GB".parse().unwrap()])?;

    Ok(())
}

pub macro fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!(*crate::i18n::LOADER, $message_id)
    }},

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!(*crate::i18n::LOADER, $message_id, $($args), *)
    }}
}

