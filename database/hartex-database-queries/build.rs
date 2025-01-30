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

pub fn main() {
    /*let api_backend_queries_path = "queries/api_backend";
    println!("cargo::rerun-if-changed={api_backend_queries_path}");

    let url = env::var("API_PGSQL_URL").unwrap();
    cornucopia::generate_live(
        &mut Client::connect(&url, NoTls).unwrap(),
        api_backend_queries_path,
        Some("generated/api_backend.rs"),
        CodegenSettings {
            derive_ser: false,
            is_async: true,
        },
    )
    .unwrap();

    let configuration_queries_path = "queries/configuration";
    println!("cargo::rerun-if-changed={configuration_queries_path}");

    let url = env::var("HARTEX_NIGHTLY_PGSQL_URL").unwrap();
    cornucopia::generate_live(
        &mut Client::connect(&url, NoTls).unwrap(),
        configuration_queries_path,
        Some("generated/configuration.rs"),
        CodegenSettings {
            derive_ser: false,
            is_async: true,
        },
    )
    .unwrap();

    let discord_frontend_queries_path = "queries/discord_frontend";
    println!("cargo::rerun-if-changed={discord_frontend_queries_path}");

    let url = env::var("HARTEX_NIGHTLY_PGSQL_URL").unwrap();
    cornucopia::generate_live(
        &mut Client::connect(&url, NoTls).unwrap(),
        discord_frontend_queries_path,
        Some("generated/discord_frontend.rs"),
        CodegenSettings {
            derive_ser: false,
            is_async: true,
        },
    )
    .unwrap();*/

    if let Err(error) =
        hartex_database_typedsql::generate_queries_with_schemas("schemas", "queries", "")
    {
        println!("cargo:error=Error while generating typed queries: {error:?}");
    }
}
