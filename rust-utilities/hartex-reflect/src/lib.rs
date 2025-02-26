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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
// #![deny(warnings)]
#![allow(unexpected_cfgs)]
#![feature(rustc_private)]
#![feature(stmt_expr_attributes)]

extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint_defs;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_target;

use std::env;

use cargo_metadata::MetadataCommand;
use itertools::Itertools;
use rustc_hir::def::DefKind;

mod rustc;

/// Generates type information for a certain crate relying on cargo and the
/// Rust compiler itself.
///
/// `crate_name` parameter has to specify a crate that is present in the dependency
/// graph of the workspace.
#[allow(clippy::missing_panics_doc)]
pub fn reflect_crate(crate_name: &str) {
    let pwd = env::current_dir().unwrap();
    let manifest = pwd.join("Cargo.toml");

    let result = MetadataCommand::new()
        .current_dir(pwd)
        .manifest_path(manifest)
        .exec();

    if result.is_err() {
        println!(
            "cargo::error=failed to run `cargo metadata` to obtain information about crate to reflect"
        );
    }

    let metadata = result.unwrap();
    let Some(reflect_pkg) = metadata
        .packages
        .into_iter()
        .find(|pkg| pkg.name == crate_name)
    else {
        unreachable!()
    };

    rustc::run_compiler_for_pkg(&reflect_pkg, |tcx| {
        let module_items = tcx.hir_crate_items(());
        let _ = module_items
            .definitions()
            .map(|ldi| (ldi, tcx.def_kind(ldi)))
            .filter(|(_, dk)| matches!(dk, DefKind::Struct))
            .map(|(ldi, _)| tcx.adt_def(ldi))
            .collect_vec();

        // todo
    });
}
