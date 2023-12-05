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

use clap::ArgMatches;
use miette::Report;
use hartex_errors::buildsystem::ProjectNotFound;

/// Runs the clean command.
#[allow(clippy::module_name_repetitions)]
pub fn clean_command(matches: &ArgMatches) -> miette::Result<()> {
    let file = hartexbuild_hartexfile::from_manifest()?;

    let project_names = matches.get_many::<String>("project").unwrap();
    let len = project_names.len();

    for (i, project_name) in project_names.enumerate() {
        println!("Cleaning {project_name} ({} / {})", i + 1, len);

        let Some(project) = file.projects.get(project_name) else {
            println!("{:?}", Report::from(ProjectNotFound {
                src: project_name.clone(),
                err_span: (0, project_name.len()).into()
            }));
            continue;
        };

        project.clean(project_name.clone())?;
    }

    Ok(())
}