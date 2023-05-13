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

//! # Test Command

use clap::ArgMatches;
use hartex_eyre::eyre::Report;

/// Runs the test command.
pub fn test_command(matches: ArgMatches) -> hartex_eyre::Result<()> {
    let file = hartexbuild_hartexfile::from_manifest()?;

    let project_name = matches.get_one::<String>("project").unwrap();
    let Some(project) = file.projects.get(project_name) else {
        return Err(Report::msg(format!("project not found: {project_name}")))
    };

    project.test(project_name.clone())?;

    Ok(())
}
