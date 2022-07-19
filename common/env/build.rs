/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

#![feature(let_else)]

use std::fs;

use toml_edit::{Document, Item, Value};

pub const BACKENDS: [&str; 1] = ["postgres"];

pub fn main() {
    let result = fs::read_to_string("../../../discord-frontend/buildconf.toml");
    if let Err(error) = &result {
        println!("cargo:warning=cannot open build configuration file: `{error}`; the `env` crate will not compile");
        return;
    }

    let config = result.unwrap();
    let config = config.trim_end().to_string();

    let result = config.parse::<Document>();
    if let Err(error) = &result {
        println!(
            "cargo:warning=invalid build configuration file: `{error}`; the `env` crate will not compile"
        );
        return;
    }
    let value = result.unwrap();

    let backend_value = value["cache"]["backend"].clone();
    let Item::Value(Value::String(backend)) = backend_value else {
        println!("cargo:warning=invalid value for `backend` value in build configuration; the `env` crate will not compile");
        return;
    };

    if !BACKENDS.contains(&backend.value().as_str()) {
        println!(
            "cargo:warning=invalid backend; must be one of: {}; the `env` crate will not compile",
            BACKENDS.join(", ")
        );
        return;
    }

    println!(
        "cargo:rustc-env=ENABLE_PGSQL_CACHE_BACKEND={}",
        backend.value().as_str() == "postgres"
    );
}
