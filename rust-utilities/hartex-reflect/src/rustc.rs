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

use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::LazyLock;
use std::sync::atomic::AtomicBool;

use cargo_metadata::Package;
use convert_case::Case;
use convert_case::Casing;
use rustc_data_structures::unord::UnordSet;
use rustc_hir::def_id::LocalDefId;
use rustc_interface::Config;
use rustc_interface::interface;
use rustc_interface::passes;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::TypeckResults;
use rustc_session::config::Input;
use rustc_session::config::Options;

static USING_INTERNAL_FEATURES: AtomicBool = AtomicBool::new(false);

pub fn run_compiler_for_pkg<F>(package: Package, cb: F)
where
    F: FnOnce(TyCtxt<'_>) + Send,
{
    let Some(Ok(lib_rs_path)) = package
        .targets
        .iter()
        .find(|targ| targ.name == package.name.to_case(Case::Snake))
        .cloned()
        .map(|targ| targ.src_path.canonicalize())
    else {
        return;
    };

    let cmd = Command::new("rustc")
        .arg("--print=sysroot")
        .output()
        .expect("failed to get sysroot from rustc");
    let maybe_sysroot = PathBuf::from_str(str::from_utf8(&cmd.stdout).unwrap()).ok();

    let conf = Config {
        opts: Options {
            maybe_sysroot,
            ..Default::default()
        },
        crate_cfg: vec![],
        crate_check_cfg: vec![],
        input: Input::File(lib_rs_path),
        output_dir: None,
        output_file: None,
        ice_file: None,
        file_loader: None,
        locale_resources: rustc_driver::DEFAULT_LOCALE_RESOURCES.to_vec(),
        lint_caps: Default::default(),
        psess_created: None,
        hash_untracked_state: None,
        register_lints: None,
        override_queries: Some(|_, providers| {
            // do nothing as more lints require running typeck
            providers.lint_mod = |_, _| {};

            // another hack so this won't try to call typeck
            providers.used_trait_imports = |_, _| {
                static NULL: LazyLock<UnordSet<LocalDefId>> = LazyLock::new(UnordSet::default);
                &NULL
            };
        }),
        make_codegen_backend: None,
        registry: rustc_driver::diagnostics_registry(),
        using_internal_features: &USING_INTERNAL_FEATURES,
        expanded_args: vec![],
    };

    interface::run_compiler(conf, |compiler| {
        let session = &compiler.sess;

        let krate = passes::parse(&session);
        rustc_interface::create_and_enter_global_ctxt(compiler, krate, |tcx| {
            if session.dcx().has_errors().is_some() {
                session.dcx().fatal("compilation failed, aborting");
            }

            if tcx.dcx().has_errors().is_some() {
                tcx.dcx().fatal("errors occurred, aborting");
            }

            cb(tcx)
        });
    });
}
