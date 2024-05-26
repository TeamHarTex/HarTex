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
use std::path::Component;
use std::path::PathBuf;
use std::sync::Arc;

use test::ColorConfig;
use test::Options;
use test::OutputFormat;
use test::RunIgnored;
use test::ShouldPanic;
use test::TestDesc;
use test::TestDescAndFn;
use test::TestFn;
use test::TestName;
use test::TestOpts;
use test::TestType;

use path_slash::PathExt;
use walkdir::WalkDir;

use crate::config::Config;
use crate::header;
use crate::header::TestsuiteOutcome;
use crate::testrunner;

#[allow(clippy::module_name_repetitions)]
pub fn run_tests(config: Arc<Config>) -> bool {
    let mut tests = Vec::new();
    discover_tests(config, &mut tests);

    let options = make_test_options();
    test::run_tests_console(&options, tests).unwrap_or(false)
}

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::needless_pass_by_value)]
fn discover_tests(config: Arc<Config>, tests: &mut Vec<TestDescAndFn>) {
    if config.ui {
        let search_dir = config.root.join("tests/ui");
        let walkdir = WalkDir::new(search_dir)
            .same_file_system(true)
            .sort_by_file_name();

        for result in walkdir {
            let entry = result.expect("failed to get entry");
            let metadata = entry.metadata().expect("failed to get entry metadata");

            if metadata.is_dir() {
                let mut components = entry.path().components();

                while let Some(component) = components.next() {
                    match component {
                        Component::Prefix(_) | Component::RootDir => continue,
                        Component::Normal(component)
                            if component != config.root.file_name().unwrap() =>
                        {
                            continue
                        }
                        _ => break,
                    }
                }

                let mut out_dir = config
                    .root
                    .join(config.build_dir.clone())
                    .join(env!("TESTSUITE_TARGET"));
                components.for_each(|component| out_dir = out_dir.join(component));

                if !out_dir.exists() {
                    fs::create_dir_all(out_dir).expect("failed to create directories");
                }
            }

            if metadata.is_symlink() {
                continue;
            }

            match entry.path().extension() {
                Some(extension) if extension != "rs" => continue,
                None => continue,
                _ => (),
            }

            if let Some(test) = make_test(config.clone(), entry.path().to_path_buf()) {
                tests.push(test);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn make_test(config: Arc<Config>, path: PathBuf) -> Option<TestDescAndFn> {
    let relative_path = path
        .strip_prefix(&config.root)
        .expect("failed to strip path prefix");

    let Ok(header) = header::parse_header(&path) else {
        eprintln!(
            "WARN: test file {} does not have a valid test file header, ignoring",
            path.display()
        );
        return None;
    };

    let testrunner_config = config.clone();
    Some(TestDescAndFn {
        desc: TestDesc {
            name: TestName::DynTestName(format!(
                "[{}] {}",
                header.testsuite_type,
                relative_path.to_slash_lossy()
            )),
            ignore: false,
            ignore_message: header.testsuite_ignoremsg,
            source_file: "",
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
            should_panic: ShouldPanic::No,
            compile_fail: header.testsuite_outcome == TestsuiteOutcome::CompileFail,
            no_run: false,
            test_type: TestType::Unknown,
        },
        testfn: TestFn::DynTestFn(Box::new(move || {
            testrunner::run(testrunner_config, path);
            Ok(())
        })),
    })
}

fn make_test_options() -> TestOpts {
    TestOpts {
        list: false,
        filters: vec![],
        filter_exact: false,
        force_run_in_process: false,
        exclude_should_panic: false,
        run_ignored: RunIgnored::No,
        run_tests: true,
        bench_benchmarks: false,
        logfile: None,
        nocapture: false,
        color: ColorConfig::AlwaysColor,
        format: OutputFormat::Terse,
        shuffle: false,
        shuffle_seed: None,
        test_threads: None,
        skip: vec![],
        time_options: None,
        fail_fast: false,
        options: Options::new(),
    }
}
