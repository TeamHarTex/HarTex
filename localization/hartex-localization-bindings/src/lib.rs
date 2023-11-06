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

#![feature(proc_macro_diagnostic)]

use hartex_localization_loader::env::base_path;
use hartex_localization_loader::load_resources;
use proc_macro::Span;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_bindings(_: TokenStream) -> TokenStream {
    let mut base_dir = base_path();
    base_dir.push("en-GB");  // todo: may not want to assume en-GB as default?

    let Ok(_) = load_resources(base_dir) else {
        Span::call_site().error(format!("failed to load localization resources from folder: {base_dir}")).emit();
        return TokenStream::new();
    };

    TokenStream::new()
}
