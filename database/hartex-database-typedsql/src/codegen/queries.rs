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

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::File;

use crate::codegen::DO_NOT_MODIFY_HEADER;
use crate::query::QueryInfo;

pub(crate) fn generate_query_structs_from_queries<P>(
    query_map: BTreeMap<String, QueryInfo>,
    path: P,
) -> crate::error::Result<()>
where
    P: AsRef<Path>,
{
    let pathref = path.as_ref().join("queries");
    let mut module_files = BTreeMap::new();

    for (name, query) in query_map {
        let path_for_query = pathref.clone().join(&query.path);
        let ident = Ident::new(name.as_str(), Span::call_site());
        module_files
            .entry(query.path)
            .or_insert(TokenStream::new())
            .append_all(quote::quote! {
                pub mod #ident;
            });

        fs::create_dir_all(&path_for_query)?;
        fs::write(
            path_for_query.join(format!("{name}.rs")),
            DO_NOT_MODIFY_HEADER,
        )?;
    }

    for (path, stream) in module_files.clone() {
        let mod_rs = pathref.clone().join(path).join("mod.rs");
        let synfile = syn::parse2::<File>(stream)?;

        fs::write(
            mod_rs,
            DO_NOT_MODIFY_HEADER.to_owned() + prettyplease::unparse(&synfile).as_str(),
        )?;
    }

    let queries_mod = pathref.join("mod.rs");
    let mods = module_files
        .keys()
        .map(|path| Ident::new(path, Span::call_site()))
        .map(|ident| quote::quote! {pub mod #ident;})
        .collect_vec();
    let queries_ts = quote::quote! {
        #(#mods)*
    };
    let file = syn::parse2::<File>(queries_ts)?;
    fs::write(
        queries_mod,
        DO_NOT_MODIFY_HEADER.to_owned() + prettyplease::unparse(&file).as_str(),
    )?;

    Ok(())
}
