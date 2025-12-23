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

use std::fmt::{Formatter, Result as FmtResult};

use rootcause::{
    ReportRef,
    handlers::FormattingFunction,
    hooks::report_formatter::ReportFormatter,
    markers::{Dynamic, Local, Uncloneable},
};
use yansi::Paint;

#[derive(Debug)]
pub struct ErrorFormatter;

impl ReportFormatter for ErrorFormatter {
    fn format_reports(
        &self,
        reports: &[ReportRef<'_, Dynamic, Uncloneable, Local>],
        fmt: &mut Formatter<'_>,
        _: FormattingFunction,
    ) -> FmtResult {
        fmt.write_str("an error occurred, see below:\n\n")?;

        // todo: ANSI doesn't work yet...
        for (i, report) in reports.iter().enumerate() {
            let msg = format!("{}: {}", i + 1, report.format_current_context());
            writeln!(fmt, "{}", msg.red())?;

            let attached = report.attachments();
            for attach in attached {
                writeln!(fmt, "  - {}", attach.format_inner().red())?;
            }
        }

        Ok(())
    }
}
