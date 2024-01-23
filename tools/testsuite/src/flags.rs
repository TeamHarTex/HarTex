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

use clap::Parser;

#[derive(Parser)]
#[clap(
    override_usage = "uitest [options]",
    disable_help_subcommand(true),
    about = "",
    next_line_help(false)
)]
pub struct Flags {
    /// Whether to run UI tests.
    #[arg(global(true), long)]
    pub ui: bool,
}

impl Flags {
    pub fn parse_from_args(args: &[String]) -> Self {
        let first = String::from("uitest");
        let iter = iter::once(&first).chain(args.iter());

        Self::parse_from(iter)
    }
}
