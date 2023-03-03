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

use std::env;
use std::process::Command;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "kebab-case")]
pub struct Project {
    pub r#type: ProjectType,
    pub profile: Option<RustBuildProfile>,
    pub include_debug_info: Option<bool>,
}

impl Project {
    pub fn build(&self, name: String) -> hartex_eyre::Result<()> {
        match self.r#type {
            ProjectType::Rust => {
                let mut pwd = env::current_dir()?;
                pwd.push(name);

                let command = Command::new("cargo")
                    .arg("build")
                    .current_dir(pwd);

                if let Some(profile) = self.profile.clone() && profile == RustBuildProfile::Release {
                    command.arg("--release");
                }

                command.status()?.exit_ok()?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Rust,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RustBuildProfile {
    Release,
}
