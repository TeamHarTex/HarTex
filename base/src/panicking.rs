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

//! Panic hook implementation utilizing `color-eyre`.

use std::fmt::{Formatter, Result};
use std::panic::PanicInfo;

use color_eyre::config::{HookBuilder, Theme};
use color_eyre::section::PanicMessage;
use owo_colors::OwoColorize;

pub(self) struct HarTexPanicMessage;

impl PanicMessage for HarTexPanicMessage {
    fn display(&self, info: &PanicInfo<'_>, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{}", "error: internal error: unexpected panic".red())?;
        writeln!(
            f,
            "{}",
            "note: the process unexpectedly panicked. this is a bug.".yellow()
        )?;
        writeln!(
            f,
            "{}",
            "note: we would appreciate a bug report: https://github.com/HarTexTeam/HarTex-rust-discord-bot/issues/new?assignees=&labels=Bot%3A+Bug%2CBot%3A+IBE&template=internal-error.yml".yellow()
        )?;
        println!();

        let payload = info
            .payload()
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| info.payload().downcast_ref::<&str>().cloned())
            .unwrap_or("<unknown panic payload>");
        write!(f, "panic payload: ")?;
        writeln!(f, "{}", payload.cyan())?;

        write!(f, "panic location: ")?;
        if let Some(location) = info.location() {
            write!(f, "{}", location.file().cyan())?;
            write!(f, " at ")?;
            write!(f, "{}", location.line().cyan())?;
            write!(f, ":")?;
            write!(f, "{}", location.column().cyan())?;
        } else {
            write!(f, "{}", "<unknown panic location>".cyan())?;
        }

        Ok(())
    }
}

/// Initializes the panic hook.
pub fn init() {
    let builder = HookBuilder::blank()
        .add_default_filters()
        .capture_span_trace_by_default(true)
        .panic_message(HarTexPanicMessage)
        .theme(Theme::dark());
    let (panic_hook, _) = builder.into_hooks();
    panic_hook.install();
}
