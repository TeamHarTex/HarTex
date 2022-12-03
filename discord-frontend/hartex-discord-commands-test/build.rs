/*
 * SPDX-License-Identifier: AGPL-3.0-only
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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::process::{Command, ExitStatus, Stdio};

fn has_cargo_expand() -> bool {
    let cargo_expand = if cfg!(windows) {
        "cargo-expand.exe"
    } else {
        "cargo-expand"
    };

    Command::new(cargo_expand)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .as_ref()
        .map(ExitStatus::success)
        .unwrap_or(false)
}

fn main() {
    if cfg!(feature = "expandtest") && has_cargo_expand() {
        println!("cargo:rustc-cfg=expandtest");
    }
}
