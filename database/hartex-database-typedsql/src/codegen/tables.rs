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

use itertools::Itertools;
use proc_macro2::Ident;
use proc_macro2::Span;
use syn::File;

use crate::schema::SchemaInfo;
use crate::schema::TableInfo;

pub(crate) struct GeneratedTableStructsFile {
    pub(crate) filename: String,
    pub(crate) content: String,
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

fn generate_token_stream(schema: SchemaInfo) -> crate::error::Result<proc_macro2::TokenStream> {
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
            let fields = generate_table_fields_token_streams(table)?;

            Ok::<proc_macro2::TokenStream, crate::error::Error>(quote::quote! {
                pub struct #ident {
                    #(#fields),*
                }
            })
        })
        .process_results(|iter| iter.collect_vec())?;

    Ok(quote::quote! {
        #(#structs)*
    })
}

fn generate_table_fields_token_streams(
    table: TableInfo,
) -> crate::error::Result<Vec<proc_macro2::TokenStream>> {
    table
        .columns
        .into_iter()
        .map(|(name, column)| {
            let ident = Ident::new(name.as_str(), Span::call_site());
            let dtype = super::types::sql_type_to_rust_type_token_stream(column.coltype)
                .ok_or(crate::error::Error::QueryFile("unsupported data type"))?;

            Ok::<proc_macro2::TokenStream, crate::error::Error>(quote::quote! {
                #ident: #dtype
            })
        })
        .process_results(|iter| iter.collect_vec())
}
