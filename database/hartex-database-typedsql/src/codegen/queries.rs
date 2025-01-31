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

use convert_case::Case;
use convert_case::Casing;
use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::File;

use crate::codegen::DO_NOT_MODIFY_HEADER;
use crate::codegen::types;
use crate::query::QueryInfo;
use crate::query::QueryInfoInner;
use crate::query::insert::InsertQueryInfo;
use crate::query::select::SelectQueryInfo;
use crate::query::select::SelectWhat;

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
            .entry(query.path.clone())
            .or_insert(TokenStream::new())
            .append_all(quote::quote! {
                pub mod #ident;
            });

        fs::create_dir_all(&path_for_query)?;

        let ts = generate_query_struct_token_stream(&name, query)?;
        let file = syn::parse2::<File>(ts)?;

        fs::write(
            path_for_query.join(format!("{name}.rs")),
            DO_NOT_MODIFY_HEADER.to_owned() + prettyplease::unparse(&file).as_str(),
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

fn generate_query_struct_token_stream(
    name: &String,
    query: QueryInfo,
) -> crate::error::Result<TokenStream> {
    let structname = Ident::new(name.to_case(Case::Pascal).as_str(), Span::call_site());

    let (table, placeholders) = match query.inner {
        QueryInfoInner::Insert(InsertQueryInfo {
            into_table,
            placeholders,
        }) => (into_table, placeholders),
        QueryInfoInner::Select(SelectQueryInfo {
            from: Some(table),
            placeholders,
            ..
        }) => (table, placeholders),
        QueryInfoInner::Select(SelectQueryInfo {
            what:
                deref!(
                    SelectWhat::Exists(SelectQueryInfo {
                        from: Some(ref table),
                        ref placeholders,
                        ..
                    })
                ),
            ..
        }) => (table.clone(), placeholders.clone()),
        _ => return Err(crate::error::Error::QueryFile("unsupported query type")),
    };

    let fields = placeholders
        .iter()
        .map(|placeholder| {
            let dtype = if let Some(col) = table.columns.get(placeholder) {
                types::sql_type_to_rust_type_token_stream(col.coltype.clone()).unwrap()
            } else if let Some(dt) = query.extra_placeholder_tys.get(placeholder) {
                types::sql_type_to_rust_type_token_stream(dt.clone()).unwrap()
            } else {
                unreachable!()
            };

            let ident = Ident::new(placeholder, Span::call_site());

            quote::quote! {
                #ident: #dtype
            }
        })
        .collect_vec();

    let bind_constructor =
        generate_query_struct_bind_constructor_token_stream(placeholders, fields.clone())?;

    Ok(quote::quote! {
        pub struct #structname {
            #(#fields),*
        }

        impl #structname {
            #bind_constructor
        }
    })
}

fn generate_query_struct_bind_constructor_token_stream(
    placeholders: Vec<String>,
    param_decls: Vec<TokenStream>,
) -> crate::error::Result<TokenStream> {
    let idents = placeholders
        .iter()
        .map(|string| Ident::new(string, Span::call_site()))
        .collect_vec();

    Ok(quote::quote! {
        #[must_use = "Queries must be executed after construction"]
        pub fn bind(#(#param_decls),*) -> Self {
            Self {
                #(#idents),*
            }
        }
    })
}
