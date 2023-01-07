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

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::panic;
use std::panic::PanicInfo;
use std::path::Path;
use std::thread;

use backtrace::Backtrace;
use backtrace::SymbolName;

use crate::constants::IGNORED_CRATES;
use crate::constants::IGNORED_SYMBOLS;

#[allow(clippy::module_name_repetitions)]
pub struct PanicHook;

impl PanicHook {
    pub fn install_hook(self) {
        panic::set_hook(self.into_panic_hook());
    }

    pub fn into_panic_hook(self) -> Box<dyn Fn(&PanicInfo<'_>) + Send + Sync + 'static> {
        Box::new(move |panic_info| eprintln!("{}", self.report(panic_info)))
    }

    #[allow(clippy::unused_self)]
    pub(crate) fn report<'a>(&'a self, panic_info: &'a PanicInfo<'a>) -> PanicReport<'a> {
        PanicReport {
            backtrace: Backtrace::new(),
            panic_info,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct PanicReport<'a> {
    backtrace: Backtrace,
    panic_info: &'a PanicInfo<'a>,
}

impl Display for PanicReport<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\r\x1B[2K")?;

        writeln!(
            f,
            "\x1B[0;31mPanic occurred in thread {:?}: {}",
            thread::current().name().unwrap_or("<unknown>"),
            self.panic_info
                .payload()
                .downcast_ref::<&str>()
                .unwrap_or(&"Box<dyn Any>")
        )?;

        for frame in self.backtrace.frames() {
            for symbol in frame.symbols() {
                let symbol_name = symbol.name().unwrap_or(SymbolName::new(b"<unknown>"));
                if IGNORED_SYMBOLS.contains(&&*format!("{symbol_name}")) {
                    continue;
                }

                if IGNORED_CRATES
                    .iter()
                    .any(|prefix| format!("{symbol_name}").contains(prefix))
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

        writeln!(f, "\nNote: this is an internal error.")?;
        writeln!(f, "Help: please report this issue at https://github.com/TeamHarTex/HarTex/issues/new?assignees=&labels=Bot%3A+Bug%2CBot%3A+Internal+Error&template=internal-error.yml")?;
        write!(f, "\x1B[0m")
    }
}
