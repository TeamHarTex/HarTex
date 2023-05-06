/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

//! # Configuration Manifest
//!
//! Specifies the configuration manifest in models that can be serialized from and deserialized
//! into.

use hartex_eyre::eyre::Report;
use hcl::eval::Context;
use hcl::eval::FuncDef;
use hcl::eval::ParamType;
use serde::Deserialize;
use serde::Serialize;

pub mod appearance;
pub mod dashboard;

/// The root of everything.
#[derive(Deserialize, Serialize)]
pub struct Configuration {
    /// Version of the configuration schema.
    pub version: u8,
    /// Dashboard access configurations.
    pub dashboard: dashboard::Dashboard,
    /// Appearance of HarTex in the server.
    pub appearance: appearance::Appearance,
}

pub fn deserialize_config(source_hcl: &str) -> hartex_eyre::Result<Configuration> {
    let mut context = Context::new();
    context.declare_func(
        "rgb",
        FuncDef::builder()
            .params([ParamType::Number, ParamType::Number, ParamType::Number])
            .build(appearance::hcl_rgb_function),
    );

    hcl::eval::from_str::<Configuration>(source_hcl, &context).map_err(Report::new)
}
