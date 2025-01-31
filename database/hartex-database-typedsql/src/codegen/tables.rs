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

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use sqlparser::ast::ColumnOption;
use syn::File;

use crate::codegen::DO_NOT_MODIFY_HEADER;
use crate::codegen::tables;
use crate::schema::SchemaInfo;
use crate::schema::TableInfo;

pub(crate) struct GeneratedTableStructsFile {
    pub(crate) filename: String,
    pub(crate) content: String,
}

pub(crate) fn generate_table_structs_from_schemas<P>(
    schemas: HashMap<String, SchemaInfo>,
    root_path: P,
) -> crate::error::Result<()>
where
    P: AsRef<Path>,
{
    let tables_dir = root_path.as_ref().join("tables");
    fs::create_dir_all(&tables_dir)?;

    let _ = schemas
        .into_iter()
        .map(tables::generate_table_structs_from_schema)
        .process_results(|iter| {
            iter.map(|file| {
                let path = tables_dir.clone().join(file.filename);

                fs::write(
                    &path,
                    DO_NOT_MODIFY_HEADER.to_owned() + file.content.as_str(),
                )?;

                Ok::<(), crate::error::Error>(())
            })
            .process_results(|iter| iter.collect_vec())
        })??;

    let mods = quote::quote! {
        pub mod api_backend;
        pub mod discord_frontend;
    };
    let file = syn::parse2::<File>(mods)?;
    fs::write(
        tables_dir.join("mod.rs"),
        DO_NOT_MODIFY_HEADER.to_owned() + prettyplease::unparse(&file).as_str(),
    )?;

    Ok(())
}

pub(crate) fn generate_table_structs_from_schema(
    (name, schema): (String, SchemaInfo),
) -> crate::error::Result<GeneratedTableStructsFile> {
    let filename = format!("{name}.rs");
    let stream = generate_token_stream(schema)?;

    let file = syn::parse2::<File>(stream)?;

    Ok(GeneratedTableStructsFile {
        filename,
        content: prettyplease::unparse(&file),
    })
}

fn generate_token_stream(schema: SchemaInfo) -> crate::error::Result<TokenStream> {
    let structs = schema
        .tables
        .clone()
        .into_iter()
        .map(|(name, table)| {
            let unquoted_name = name
                .replace("public.", "")
                .replace('"', "")
                .replace('.', "");
            let ident = Ident::new(unquoted_name.as_str(), Span::call_site());
            let fields = generate_table_fields_token_streams(table.clone())?;
            let impl_block = generate_struct_impl_token_stream(ident.clone(), table)?;

            Ok::<TokenStream, crate::error::Error>(quote::quote! {
                pub struct #ident {
                    #(#fields),*
                }

                #impl_block
            })
        })
        .process_results(|iter| iter.collect_vec())?;

    Ok(quote::quote! {
        #(#structs)*
    })
}

fn generate_table_fields_token_streams(table: TableInfo) -> crate::error::Result<Vec<TokenStream>> {
    table
        .columns
        .into_iter()
        .map(|(name, column)| {
            let ident = Ident::new(name.as_str(), Span::call_site());
            let dtype = super::types::sql_type_to_rust_type_token_stream(column.coltype)
                .ok_or(crate::error::Error::QueryFile("unsupported data type"))?;

            Ok::<TokenStream, crate::error::Error>(
                if column.constraints.contains(&ColumnOption::NotNull) {
                    quote::quote! {
                        #ident: #dtype
                    }
                } else {
                    quote::quote! {
                        #ident: Option<#dtype>
                    }
                },
            )
        })
        .process_results(|iter| iter.collect_vec())
}

fn generate_struct_impl_token_stream(
    ident: Ident,
    table: TableInfo,
) -> crate::error::Result<TokenStream> {
    let tss = table
        .columns
        .into_iter()
        .map(|(name, column)| {
            let fn_name = Ident::new(name.as_str(), Span::call_site());
            let dtype = super::types::sql_type_to_rust_reftype_token_stream(column.coltype)
                .ok_or(crate::error::Error::QueryFile("unsupported data type"))?;
            let rettype = if column.constraints.contains(&ColumnOption::NotNull) {
                quote::quote! {#dtype}
            } else {
                quote::quote! {Option<#dtype>}
            };
            let body = generate_getter_body(fn_name.clone(), rettype.to_string())?;

            Ok::<TokenStream, crate::error::Error>(quote::quote! {
                #[must_use]
                pub fn #fn_name(&self) -> #rettype {
                    #body
                }
            })
        })
        .process_results(|iter| iter.collect_vec())?;

    Ok(quote::quote! {
        impl #ident {
            #(#tss)*
        }
    })
}

fn generate_getter_body(name: Ident, ty: String) -> crate::error::Result<TokenStream> {
    Ok(match ty.as_str() {
        "& str" => quote::quote! {self.#name.as_str()},
        "Option < & str >" => quote::quote! {self.#name.as_deref()},
        "& [String]" => quote::quote! {self.#name.as_slice()},
        _ => quote::quote! {self.#name},
    })
}
