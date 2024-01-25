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
use std::fs::OpenOptions;
use std::io::Write;
use std::panic;
use std::path::Component;
use std::path::Path;
use std::process::Command;

use crate::config::Config;
use crate::testrunner::diff;

#[allow(clippy::module_name_repetitions)]
pub struct TestContext<'test> {
    pub config: &'test Config,
    pub path: &'test Path,
}

impl<'test> TestContext<'test> {
    #[must_use]
    pub fn new(config: &'test Config, test_path: &'test Path) -> Self {
        Self {
            config,
            path: test_path,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::unused_io_amount)]
    pub fn run_ui_test(&self) {
        let from_test = self
            .path
            .strip_prefix(&self.config.root)
            .expect("failed to extract relative path");
        let workspace_test = from_test
            .strip_prefix("tests/ui")
            .expect("failed to extract workspace path");

        let Component::Normal(workspace) = workspace_test
            .components()
            .next()
            .expect("unexpected end of path")
        else {
            unreachable!()
        };

        let mut command = Command::new("rustc");
        command.arg(from_test);

        // rustc codegen flags
        command.args([
            "-Ccodegen-units=1",
            "-Cstrip=debuginfo",
            "-Zthreads=1",
            "-Zui-testing",
            "-Zdeduplicate-diagnostics=no",
            "-Zwrite-long-types-to-disk=no",
        ]);

        // library search path
        command.arg(format!(
            "-L all={}",
            self.config
                .build_dir
                .join(env!("TESTSUITE_TARGET"))
                .join(workspace)
                .join("debug")
                .display()
        ));

        command.current_dir(&self.config.root);
        let output = command.output().expect("failed to get output");
        let output_str = String::from_utf8(output.stderr).expect("invalid utf-8 detected");

        let mut expected_path = self.path.to_path_buf();
        expected_path.set_extension("stderr");
        let expected_str = fs::read_to_string(&expected_path).expect("failed to read file");

        if !diff::compare_lines_and_render_if_needed(&expected_str, &output_str) {
            let expected_from_test = expected_path
                .strip_prefix(&self.config.root)
                .expect("failed to extract relative path");
            let write_path = self.config.root.join(
                self.config
                    .build_dir
                    .join(env!("TESTSUITE_TARGET"))
                    .join(expected_from_test),
            );

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&write_path)
                .expect("failed to open file");
            file.write(output_str.as_bytes())
                .expect("failed to write to file");

            println!("Actual output differed from the expected.");
            println!(
                "The actual output has been written to {}",
                write_path.display()
            );

            panic::resume_unwind(Box::new(()));
        }
    }
}
