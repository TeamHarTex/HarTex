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

use std::process::Command;
use std::process::exit;

use owo_colors::OwoColorize;

use crate::builder::Builder;
use crate::builder::RunConfig;
use crate::builder::Step;

/// Step for building the api-backend project.
pub struct Api;

impl Step for Api {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        build_cargo_project("api-backend", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("api-backend"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(Api);
        }
    }
}

/// Step for building the hartex-database-queries project.
pub struct Database;

impl Step for Database {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        build_cargo_project("database", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("database"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(Database);
        }
    }
}

/// Step for building the discord-frontend project.
pub struct Discord;

impl Step for Discord {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        build_cargo_project("discord-frontend", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("discord-frontend"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(Discord);
        }
    }
}

/// Step for building the localization project.
pub struct Localization;

impl Step for Localization {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        build_cargo_project("localization", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("localization"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(Localization);
        }
    }
}

/// Step for building the rust-utilities project.
pub struct Utilities;

impl Step for Utilities {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        build_cargo_project("rust-utilities", builder);
    }

    fn run_config(run: RunConfig<'_>) {
        if run
            .builder
            .config
            .subcommand_args
            .contains(&String::from("rust-utilities"))
            || run.builder.config.subcommand_args.is_empty()
        {
            run.builder.run_step(Utilities);
        }
    }
}

/// Utility function for building a project using `cargo build`.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::module_name_repetitions)]
pub fn build_cargo_project(project: &'static str, builder: &Builder<'_>) {
    let pwd = builder.config.root.join(project);

    let mut command = Command::new("cargo");
    command.arg("build");

    let mut rustflags = format!(
        "-C opt-level={} --cfg hartexbootstrap",
        builder.config.opt_level
    );

    if builder.config.debug {
        rustflags.push_str(" -g");
    }

    if builder.config.release {
        command.arg("--release");
    }

    command.current_dir(pwd);
    command.env(
        "CARGO_TARGET_DIR",
        builder
            .config
            .root
            .join(builder.config.output_dir.clone())
            .join(env!("BOOTSTRAP_TARGET"))
            .join(project),
    );
    command.env("RUSTFLAGS", rustflags);

    println!("{} Building {project} project", "info:".bold());
    let status = command.status().expect("failed to get status");
    if !status.success() {
        exit(status.code().unwrap_or(1));
    }
}
