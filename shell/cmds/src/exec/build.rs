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

use std::process::Command;

pub fn run_build(options: Vec<&str>) {
    let Some(branch) = options.get(0) else {
        println!("build: \x1B[1;31merror: \x1B[0m\x1B[1mmissing branch to compile\x1B[0m");
        return;
    };

    println!("build: checking out branch {branch}");
    if let Err(error) = Command::new("git")
        .arg("checkout")
        .arg(branch)
        .status() {
        println!("build: \x1B[1;31merror: \x1B[0m\x1B[1mfailed to checkout branch: {error}\x1B[0m");
        return;
    }
}
