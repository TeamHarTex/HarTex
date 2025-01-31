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

use std::fs;
use std::path::Path;

use syn::File;

use crate::codegen::DO_NOT_MODIFY_HEADER;

pub(crate) fn generate_lib_rs<P>(path: P) -> crate::error::Result<()>
where
    P: AsRef<Path>,
{
    let ts = quote::quote! {
        #![deny(clippy::pedantic)]
        #![deny(unsafe_code)]
        #![deny(warnings)]

        #[path = "../generated/api_backend.rs"]
        pub mod api_backend;
        #[path = "../generated/configuration.rs"]
        pub mod configuration;
        #[path = "../generated/discord_frontend.rs"]
        pub mod discord_frontend;

        pub mod queries;
        pub mod result;
        pub mod tables;
    };

    let synfile = syn::parse2::<File>(ts)?;

    fs::write(
        path.as_ref().join("lib.rs"),
        DO_NOT_MODIFY_HEADER.to_owned() + prettyplease::unparse(&synfile).as_str(),
    )?;

    Ok(())
}
