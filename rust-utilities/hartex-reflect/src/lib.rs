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

#![feature(rustc_private)]

extern crate rustc_interface;

use std::env;
use std::sync::Arc;

use cargo::GlobalContext;
use cargo::core::Workspace;
use cargo::core::compiler::CompileMode;
use cargo::core::compiler::Executor;
use cargo::ops::CompileOptions;
use cargo::ops::compile;

/// Generates type information for a certain crate relying on cargo and the
/// Rust compiler itself.
///
/// `crate_name` parameter has to specify a crate that is present in the dependency
/// graph of the workspace.
pub fn reflect_crate(crate_name: &str) {
    let pwd = env::current_dir().unwrap();
    let manifest = pwd.join("Cargo.toml");

    let ctx = GlobalContext::default().unwrap();
    let ws = Workspace::new(manifest.as_path(), &ctx).unwrap();

    //let executor: Arc<dyn Executor> = Arc::new(ReflectExecutor::new(crate_name));
    let mut options = CompileOptions::new(&ctx, CompileMode::Build).unwrap();
    options.build_config.build_plan = true;
    options.build_config.dry_run = true;
    /*let _ = compile(
        &ws, &options,
        //&executor,
    );*/
}
