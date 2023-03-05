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

use hartex_eyre::eyre::Report;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "kebab-case")]
pub struct Project {
    pub r#type: ProjectType,
    pub profile: Option<RustBuildProfile>,
    pub include_debug_info: Option<bool>,
    pub package_manager: Option<JsTsPackageManager>,
}

impl Project {
    pub fn build(&self, name: String) -> hartex_eyre::Result<()> {
        let mut pwd = env::current_dir()?;
        pwd.push(name);

        let result = match self.r#type {
            ProjectType::JsTs => {
                let package_manager = self.package_manager.clone().ok_or(Report::msg(
                    "package manager not specified for jsts project",
                ))?;
                let mut command = package_manager.into_command();
                command.current_dir(pwd).arg("build");
                command.status()?.exit_ok()
            }
            ProjectType::Rust => {
                let mut command = Command::new("cargo");
                command.arg("build").current_dir(pwd);

                if let Some(profile) = self.profile.clone() && profile == RustBuildProfile::Release {
                    command.arg("--release");
                }

                if let Some(include_debug_info) = self.include_debug_info && include_debug_info {
                    command.env("RUSTFLAGS", "-g");
                }

                command.status()?.exit_ok()
            }
        };

        result.map_err(|error| Report::msg(format!("abnormal termination: {error}")))
    }

    pub fn lint(&self, name: String) -> hartex_eyre::Result<()> {
        let mut pwd = env::current_dir()?;
        pwd.push(name);

        let result = match self.r#type {
            ProjectType::JsTs => {
                let package_manager = self.package_manager.clone().ok_or(Report::msg(
                    "package manager not specified for jsts project",
                ))?;
                let mut command = package_manager.into_command();
                command.arg("eslint").current_dir(pwd);
                command.status()?.exit_ok()
            }
            ProjectType::Rust => {
                let mut command = Command::new("cargo");
                command.arg("clippy").current_dir(pwd.clone());
                command.status()?.exit_ok()?;

                let mut command = Command::new("cargo");
                command.arg("fmt").current_dir(pwd);
                command.status()?.exit_ok()
            }
        };

        result.map_err(|error| Report::msg(format!("abnormal termination: {error}")))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    JsTs,
    Rust,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsTsPackageManager {
    Npm,
    Yarn,
}

impl JsTsPackageManager {
    pub fn into_command(self) -> Command {
        let program = match self {
            Self::Npm => "npm",
            Self::Yarn => "yarn",
        };

        Command::new(program)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RustBuildProfile {
    Release,
}
