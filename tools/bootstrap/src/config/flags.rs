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

use std::iter;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[clap(
    override_usage = "x.py <subcommand> [options]",
    disable_help_subcommand(true),
    about = "",
    next_line_help(false)
)]
pub struct Flags {
    #[command(subcommand)]
    pub subcommand: BootstrapSubcommand,
    /// Whether to bypass locking the build process. This flag is rarely used.
    #[arg(global(true), long)]
    pub bypass_fs_lock: bool,
    /// Produce JSON output for `x.py check`.
    #[arg(global(true), long)]
    pub json: bool,
    /// Arguments passed to the subcommand.
    #[arg(global(true), last(true), value_name = "args")]
    pub subcommand_args: Vec<String>,
}

impl Flags {
    /// Parse the flags from command line arguments.
    #[must_use]
    pub fn parse_from_args(args: &[String]) -> Self {
        let first = String::from("x.py");
        let iter = iter::once(&first).chain(args.iter());

        Self::parse_from(iter)
    }
}

#[derive(Clone, Debug, Subcommand)]
pub enum BootstrapSubcommand {
    /// Compile a specified list of projects.
    Build,
    /// Checks a specified list of projects via `cargo check`.
    Check,
    /// Cleans build artifact directory for a specified list of projects.
    Clean,
    /// Runs `cargo clippy` on a specified list of projects.
    Clippy,
    /// Setup the development enrivonment.
    Setup,
    /// Runs specified tests.
    Test,
}
