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

use std::fmt;
use std::fmt::Display;
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

pub struct Header {
    pub testsuite_ignore: TestsuiteIgnore,
    pub testsuite_type: TestsuiteType,
    pub testsuite_outcome: TestsuiteOutcome,
}

impl Header {
    pub fn new(ignore: TestsuiteIgnore, r#type: TestsuiteType, outcome: TestsuiteOutcome) -> Self {
        Self {
            testsuite_ignore: ignore,
            testsuite_type: r#type,
            testsuite_outcome: outcome,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TestsuiteIgnore {
    Always,
    Never,
}

impl Display for TestsuiteIgnore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => f.write_str("always"),
            Self::Never => f.write_str("never"),
        }
    }
}

impl FromStr for TestsuiteIgnore {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            _ => Ok(Self::Never),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TestsuiteType {
    Ui,
    Unknown,
}

impl Display for TestsuiteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ui => f.write_str("ui"),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}

impl FromStr for TestsuiteType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ui" => Ok(Self::Ui),
            _ => Ok(Self::Unknown),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TestsuiteOutcome {
    CompileFail,
    Unknown,
}

impl FromStr for TestsuiteOutcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ui" => Ok(Self::CompileFail),
            _ => Ok(Self::Unknown),
        }
    }
}

pub fn parse_header(file_path: &Path) -> io::Result<Header> {
    let content = fs::read_to_string(file_path)?;
    let mut lines = content.lines();

    if lines.clone().peekable().peek().is_none() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "unexpected end of file",
        ));
    }

    // check first line for the `==BEGIN TESTSUITE DECL==`
    if let Some(line) = lines.next()
        && line != "// ==BEGIN TESTSUITE DECL=="
    {
        return Err(io::Error::new(
            io::ErrorKind::Uncategorized,
            "begin decl line `==BEGIN TESTSUITE DECL==` not found",
        ));
    }

    let mut header = Header::new(
        TestsuiteIgnore::Never,
        TestsuiteType::Unknown,
        TestsuiteOutcome::Unknown,
    );

    while let Some(line) = lines.next()
        && line != "// ==END TESTSUITE DECL=="
    {
        if !line.starts_with("//") {
            return Err(io::Error::new(
                io::ErrorKind::Uncategorized,
                "invalid line after begin decl, expected comment",
            ));
        }

        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(io::Error::new(
                io::ErrorKind::Uncategorized,
                "invalid line after begin decl, expected 3 whitespace-separated parts",
            ));
        }

        match parts[1] {
            "testsuite-ignore:" => {
                header.testsuite_ignore = TestsuiteIgnore::from_str(parts[2]).unwrap()
            }
            "testsuite-type:" => header.testsuite_type = TestsuiteType::from_str(parts[2]).unwrap(),
            "testsuite-outcome:" => {
                header.testsuite_outcome = TestsuiteOutcome::from_str(parts[2]).unwrap()
            }
            _ => (),
        }
    }

    Ok(header)
}
