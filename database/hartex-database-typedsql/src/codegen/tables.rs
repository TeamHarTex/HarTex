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

use proc_macro2::Ident;
use proc_macro2::Span;

use crate::schema::SchemaInfo;
use crate::schema::TableInfo;

pub(crate) struct GeneratedTableStructsFile {
    pub(crate) filename: String,
    pub(crate) content: String,
}

pub(crate) fn generate_table_structs_from_schema(
    (name, schema): (String, SchemaInfo),
) -> GeneratedTableStructsFile {
    let _ = format!("{name}.rs");
    let _ = generate_token_stream(schema);

    todo!()
}

fn generate_token_stream(schema: SchemaInfo) -> proc_macro2::TokenStream {
    let _ = schema
        .tables
        .clone()
        .into_iter()
        .map(|(name, table)| {
            let unquoted_name = name
                .replace("public.", "")
                .replace('"', "")
                .replace('.', "");
            let ident = Ident::new(unquoted_name.as_str(), Span::call_site());
            let _ = generate_table_fields_token_streams(table);

            quote::quote! {
                pub struct #ident {

                }
            }
        })
        .collect::<Vec<_>>();

    todo!()
}

fn generate_table_fields_token_streams(table: TableInfo) -> Vec<proc_macro2::TokenStream> {
    table.columns.into_iter().map(|(name, _)| {
        let ident = Ident::new(name.as_str(), Span::call_site());

        quote::quote! {
            #ident:
        }
    }).collect()
}
