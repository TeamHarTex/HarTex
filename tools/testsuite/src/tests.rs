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
use std::sync::Arc;

use walkdir::WalkDir;

use crate::config::Config;

pub fn discover_tests(config: Arc<Config>, tests: &mut Vec<test::TestDescAndFn>) {
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
                        Component::RootDir => continue,
                        Component::Normal(component) if component != config.root.file_name().unwrap() => continue,
                        _ => break,
                    }
                }

                let mut out_dir = config.root.join(config.build_dir.clone()).join(env!("TESTSUITE_TARGET"));
                components.for_each(|component| out_dir = out_dir.join(component));

                if !out_dir.exists() {
                    fs::create_dir_all(out_dir).expect("failed to create directories");
                }
            }
        }
    }
}

pub fn run_tests(config: Arc<Config>) {
    let mut tests = Vec::new();
    discover_tests(config, &mut tests);
}
