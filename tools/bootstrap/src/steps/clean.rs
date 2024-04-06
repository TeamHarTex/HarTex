/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

use std::fs;

use crate::builder::Builder;
use crate::builder::RunConfig;
use crate::builder::Step;

/// Step for cleaning the api-backend built artifacts.
#[allow(clippy::module_name_repetitions)]
pub struct CleanApi;

impl Step for CleanApi {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        clean("api-backend", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("api-backend"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(CleanApi);
        }
    }
}

/// Step for cleaning the database-queries built artifacts.
#[allow(clippy::module_name_repetitions)]
pub struct CleanDatabase;

impl Step for CleanDatabase {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        clean("database-queries", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("database-queries"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(CleanDatabase);
        }
    }
}

/// Step for cleaning the discord-frontend built artifacts.
#[allow(clippy::module_name_repetitions)]
pub struct CleanDiscord;

impl Step for CleanDiscord {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        clean("discord-frontend", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("discord-frontend"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(CleanDiscord);
        }
    }
}

/// Step for cleaning the localization built artifacts.
#[allow(clippy::module_name_repetitions)]
pub struct CleanLocalization;

impl Step for CleanLocalization {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        clean("localization", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("localization"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(CleanLocalization);
        }
    }
}

/// Step for cleaning the rust-utilities built artifacts.
#[allow(clippy::module_name_repetitions)]
pub struct CleanUtilities;

impl Step for CleanUtilities {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        clean("rust-utilities", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("rust-utilities"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(CleanUtilities);
        }
    }
}

/// Utility function for cleaning the built artifact directories of a given project.
#[allow(clippy::missing_panics_doc)]
fn clean(project: &'static str, builder: &Builder<'_>) {
    let dir = builder
        .config
        .root
        .join(builder.config.output_dir.clone())
        .join(env!("BOOTSTRAP_TARGET"))
        .join(project);

    println!("INFO: deleting {}", dir.display());
    if !dir.exists() {
        println!("WARN: directory {} does not exist, skipping", dir.display());
        return;
    }

    fs::remove_dir_all(dir).expect("failed to remove directory");
}
