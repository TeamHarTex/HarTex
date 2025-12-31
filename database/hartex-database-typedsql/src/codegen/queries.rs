/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use std::{collections::BTreeMap, fs, path::Path};

use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::TokenStreamExt;
use sqlparser::ast::ColumnOption;
use syn::File;

use crate::{
    codegen::{DO_NOT_MODIFY_HEADER, types},
    query::{
        QueryInfo, QueryInfoInner,
        insert::InsertQueryInfo,
        select::{SelectQueryInfo, SelectWhat},
    },
};

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

        let ts = generate_query_struct_token_stream(&name, &query)?;
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
    query: &QueryInfo,
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

    let bind_params = placeholders
        .iter()
        .map(|placeholder| {
            let dtype = if let Some(col) = table.columns.get(placeholder) {
                let dt = types::sql_type_to_rust_type_token_stream(&col.coltype).unwrap();

                if col.constraints.contains(&ColumnOption::NotNull) {
                    dt
                } else {
                    quote::quote! {Option<#dt>}
                }
            } else if let Some(dt) = query.extra_placeholder_tys.get(placeholder) {
                types::sql_type_to_rust_type_token_stream(dt).unwrap()
            } else {
                unreachable!()
            };

            let ident = Ident::new(placeholder, Span::call_site());

            quote::quote! {
                #ident: #dtype
            }
        })
        .collect_vec();
    let mut rettype = TokenStream::new();
    let query_fns = generate_query_fns_token_streams(query.clone(), &query.path, &mut rettype);

    let query_type = if rettype.is_empty() {
        quote::quote! {Query<'a, Postgres, PgArguments>}
    } else {
        quote::quote! {QueryAs<'a, Postgres, #rettype, PgArguments>}
    };

    let bind_fn = generate_bind_fn_token_stream(query.clone(), !rettype.is_empty(), &bind_params);

    Ok(quote::quote! {
        use sqlx::Postgres;
        use sqlx::postgres::PgArguments;
        use sqlx::postgres::PgPool;
        use sqlx::query::Query;
        use sqlx::query::QueryAs;

        use crate::result::IntoCrateResult;

        pub struct #structname<'a> {
            pool: &'a PgPool,
            query: Option<#query_type>
        }

        impl<'a> #structname<'a> {
            pub fn new(pool: &'a PgPool) -> Self {
                Self {
                    pool,
                    query: None,
                }
            }

            #bind_fn

            #(#query_fns)*
        }
    })
}

fn generate_bind_fn_token_stream(
    query_info: QueryInfo,
    is_query_as: bool,
    bind_params: &Vec<TokenStream>,
) -> TokenStream {
    let mut rawstr = query_info.raw.to_string();
    let placeholders = match query_info.inner {
        QueryInfoInner::Insert(insert) => insert.placeholders,
        QueryInfoInner::Select(select) => select.placeholders,
    };

    for (i, placeholder) in placeholders.iter().enumerate() {
        rawstr = rawstr.replace(&format!(":{placeholder}"), &format!("${}", i + 1));
    }
    let stmt = Literal::string(rawstr.as_str());

    let placeholder_binding = placeholders
        .iter()
        .map(|placeholder| Ident::new(placeholder, Span::call_site()))
        .map(|ident| quote::quote! {.bind(#ident)})
        .collect_vec();

    let sqlx_call = if is_query_as {
        quote::quote! {sqlx::query_as(#stmt)}
    } else {
        quote::quote! {sqlx::query(#stmt)}
    };

    quote::quote! {
        pub fn bind(mut self, #(#bind_params),*) -> Self {
            self.query.replace(#sqlx_call #(#placeholder_binding)*);
            self
        }
    }
}

fn generate_query_fns_token_streams(
    query_info: QueryInfo,
    schema: &str,
    rettype: &mut TokenStream,
) -> Vec<TokenStream> {
    match query_info.inner {
        QueryInfoInner::Insert(_) => generate_insert_query_fn_token_stream(),
        QueryInfoInner::Select(select) => {
            generate_select_query_fns_token_streams(&select, schema, rettype)
        }
    }
}

fn generate_insert_query_fn_token_stream() -> Vec<TokenStream> {
    vec![quote::quote! {
        pub async fn execute(self) -> crate::result::Result<()> {
            self.query.ok_or(crate::result::Error::Generic(".bind() has not been called on this query yet"))?
                .execute(self.pool)
                .await
                .into_crate_result()?;

            Ok(())
        }
    }]
}

fn generate_select_query_fns_token_streams(
    select: &SelectQueryInfo,
    schema: &str,
    rettype_out: &mut TokenStream,
) -> Vec<TokenStream> {
    let schemaident = Ident::new(schema.to_case(Case::Snake).as_str(), Span::call_site());
    let rettype = match select.what {
        deref!(SelectWhat::Everything) => {
            let table = select.from.as_ref().unwrap();
            let name = table.name.replace("public.", "").replace(['"', '.'], "");
            let ident = Ident::new(&name, Span::call_site());

            quote::quote! {crate::tables::#schemaident::#ident}
        }
        deref!(SelectWhat::Exists(_)) => {
            return special_token_stream_for_select_exists(&quote::quote! {bool});
        }
        _ => return vec![],
    };

    rettype_out.append_all(rettype.clone());

    vec![
        quote::quote! {
            pub async fn one(self) -> crate::result::Result<#rettype> {
                self.query.ok_or(crate::result::Error::Generic(".bind() has not been called on this query yet"))?
                    .fetch_one(self.pool)
                    .await
                    .into_crate_result()
            }
        },
        quote::quote! {
            pub async fn all(self) -> crate::result::Result<Vec<#rettype>> {
                self.query.ok_or(crate::result::Error::Generic(".bind() has not been called on this query yet"))?
                    .fetch_all(self.pool)
                    .await
                    .into_crate_result()
            }
        },
    ]
}

fn special_token_stream_for_select_exists(rettype: &TokenStream) -> Vec<TokenStream> {
    vec![quote::quote! {
        #[must_use = "Query result(s) must be used"]
        pub async fn exists(self) -> crate::result::Result<#rettype> {
            use sqlx::Row;

            Ok(self.query.ok_or(crate::result::Error::Generic(".executor() has not been called on this query yet"))?
                .fetch_one(self.pool)
                .await
                .into_crate_result()?
                .get::<#rettype, &str>("exists"))
        }
    }]
}
