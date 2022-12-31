/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::path::Path;
use std::thread;

use backtrace::Backtrace;
use backtrace::SymbolName;

#[allow(clippy::module_name_repetitions)]
pub struct HookHandler {
    pub(crate) backtrace: Backtrace,
}

impl eyre::EyreHandler for HookHandler {
    fn debug(&self, error: &(dyn Error + 'static), f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\r\x1B[2K")?;
        writeln!(
            f,
            "\x1B[0;31mError occurred in thread {:?}: {error:?}",
            thread::current().name().unwrap_or("<unknown>")
        )?;

        for frame in self.backtrace.frames() {
            for symbol in frame.symbols() {
                let symbol_name = symbol.name().unwrap_or(SymbolName::new(b"<unknown>"));
                if [
                    "RtlUserThreadStart",
                    "BaseThreadInitThunk",
                    "__scrt_common_main_seh",
                    "invoke_main",
                    "main",
                ]
                .contains(&&*format!("{symbol_name}"))
                {
                    continue;
                }

                if ["std", "core", "eyre", "backtrace", "hartex_eyre"]
                    .iter()
                    .any(|prefix| format!("{symbol_name}").starts_with(prefix))
                {
                    continue;
                }

                write!(f, "        at {symbol_name}",)?;
                writeln!(
                    f,
                    "({}:{})",
                    symbol
                        .filename()
                        .map_or(Path::new("<unknown>").display(), Path::display),
                    symbol
                        .lineno()
                        .map_or(String::from("<unknown>"), |lineno| lineno.to_string())
                )?;
            }
        }

        write!(f, "\x1B[0m")
    }
}
