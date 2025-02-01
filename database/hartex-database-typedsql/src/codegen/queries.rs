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
use proc_macro2::Literal;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use sqlparser::ast::Statement;
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

    let (table, placeholders) = match query.inner.clone() {
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

    let bind_constructor_and_executor =
        generate_query_struct_bind_constructor_and_executor_token_stream(
            placeholders,
            fields.clone(),
            &query.path,
        )?;
    let query_fns = generate_query_fns_token_streams(query.clone(), &query.path)?;

    Ok(quote::quote! {
        use std::env;

        use itertools::Itertools;
        use tokio::net::TcpStream;
        use wtx::database::Executor as _;
        use wtx::database::Records;
        use wtx::database::client::postgres::Executor;
        use wtx::database::client::postgres::ExecutorBuffer;
        use wtx::misc::Uri;

        use crate::result::IntoCrateResult;

        pub struct #structname {
            db_executor: Option<Executor<wtx::Error, ExecutorBuffer, TcpStream>>,
            executor_constructor: for<'a> fn(Uri<&'a str>) -> crate::internal::Ret<'a>,

            #(#fields),*
        }

        impl #structname {
            #bind_constructor_and_executor

            #(#query_fns)*
        }
    })
}

fn generate_query_struct_bind_constructor_and_executor_token_stream(
    placeholders: Vec<String>,
    param_decls: Vec<TokenStream>,
    schema_for_env: &str,
) -> crate::error::Result<TokenStream> {
    let idents = placeholders
        .iter()
        .map(|string| Ident::new(string, Span::call_site()))
        .collect_vec();
    let envvarraw = format!("{}_PGSQL_URL", schema_for_env.to_case(Case::Constant));
    let lit = Literal::string(envvarraw.as_str());

    Ok(quote::quote! {
        #[must_use = "Queries must be executed after construction"]
        pub fn bind(#(#param_decls),*) -> Self {
            Self {
                db_executor: None,
                executor_constructor: crate::internal::__internal_executor_constructor as for<'a> fn(Uri<&'a str>) -> crate::internal::Ret<'a>,

                #(#idents),*
            }
        }

        pub async fn executor(mut self) -> crate::result::Result<Self> {
            self.db_executor.replace((self.executor_constructor)(Uri::new(&env::var(#lit).unwrap())).await?);
            Ok(self)
        }
    })
}

fn generate_query_fns_token_streams(
    query_info: QueryInfo,
    schema: &str,
) -> crate::error::Result<Vec<TokenStream>> {
    match query_info.inner {
        QueryInfoInner::Insert(insert) => {
            generate_insert_query_fn_token_stream(insert, query_info.raw)
        }
        QueryInfoInner::Select(select) => {
            generate_select_query_fns_token_streams(select, query_info.raw, schema)
        }
    }
}

fn generate_insert_query_fn_token_stream(
    insert: InsertQueryInfo,
    raw: Statement,
) -> crate::error::Result<Vec<TokenStream>> {
    let mut rawstr = raw.to_string();
    for (i, placeholder) in insert.placeholders.iter().enumerate() {
        rawstr = rawstr.replace(&format!(":{placeholder}"), &format!("${}", i + 1));
    }
    let stmt = Literal::string(rawstr.as_str());

    let placeholders = insert
        .placeholders
        .iter()
        .map(|placeholder| Ident::new(placeholder, Span::call_site()))
        .map(|ident| quote::quote! {self.#ident})
        .collect_vec();

    Ok(vec![quote::quote! {
        pub async fn execute(self) -> crate::result::Result<u64> {
            self.db_executor.ok_or(crate::result::Error::Generic(".executor() has not been called on this query yet"))?
                .execute_with_stmt(#stmt, (#(#placeholders),* ,)).await.into_crate_result()
        }
    }])
}

fn generate_select_query_fns_token_streams(
    select: SelectQueryInfo,
    raw: Statement,
    schema: &str,
) -> crate::error::Result<Vec<TokenStream>> {
    let mut rawstr = raw.to_string();
    for (i, placeholder) in select.placeholders.iter().enumerate() {
        rawstr = rawstr.replace(&format!(":{placeholder}"), &format!("${}", i + 1));
    }
    let stmt = Literal::string(rawstr.as_str());

    let placeholders = select
        .placeholders
        .iter()
        .map(|placeholder| Ident::new(placeholder, Span::call_site()))
        .map(|ident| quote::quote! {self.#ident})
        .collect_vec();

    let schemaident = Ident::new(schema.to_case(Case::Snake).as_str(), Span::call_site());
    let rettype = match select.what {
        deref!(SelectWhat::Everything) => {
            let table = select.from.unwrap();
            let name = table
                .name
                .replace("public.", "")
                .replace('"', "")
                .replace('.', "");
            let ident = Ident::new(&name, Span::call_site());

            quote::quote! {crate::tables::#schemaident::#ident}
        }
        deref!(SelectWhat::Boolean(_)) => quote::quote! {bool},
        _ => return Ok(vec![]),
    };

    Ok(vec![
        quote::quote! {
            pub async fn one(self) -> crate::result::Result<#rettype> {
                self.db_executor.ok_or(crate::result::Error::Generic(".executor() has not been called on this query yet"))?
                    .fetch_with_stmt(#stmt, (#(#placeholders),* ,))
                    .await
                    .into_crate_result()
                    .map(|record| #rettype::try_from(record))
                    .flatten()
            }
        },
        /*quote::quote! {
            pub async fn many(self) -> crate::result::Result<Vec<#rettype>> {
                self.db_executor.ok_or(crate::result::Error::Generic(".executor() has not been called on this query yet"))?
                    .fetch_many_with_stmt(#stmt, (#(#placeholders),* ,))
                    .await
                    .into_crate_result()?
                    .iter()
                    .map(|record| #rettype::try_from(record))
                    .process_results(|iter| iter.collect_vec())
            }
        },*/
    ])
}
