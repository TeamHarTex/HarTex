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

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::panic::{self, PanicInfo};
use std::path::Path;
use std::thread;

use backtrace::{Backtrace, SymbolName};

pub struct PanicHook;

impl PanicHook {
    pub fn install_hook(self) {
        panic::set_hook(self.into_panic_hook())
    }

    pub fn into_panic_hook(self) -> Box<dyn Fn(&PanicInfo<'_>) + Send + Sync + 'static> {
        Box::new(move |panic_info| eprintln!("{}", self.report(panic_info)))
    }

    pub(crate) fn report<'a>(&'a self, panic_info: &'a PanicInfo<'a>) -> PanicReport<'a> {
        PanicReport {
            backtrace: Backtrace::new(),
            panic_info,
        }
    }
}

pub struct PanicReport<'a> {
    backtrace: Backtrace,
    panic_info: &'a PanicInfo<'a>,
}

impl Display for PanicReport<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
                        .map(|path| path.display())
                        .unwrap_or(Path::new("<unknown>").display()),
                    symbol
                        .lineno()
                        .map(|lineno| lineno.to_string())
                        .unwrap_or(String::from("<unknown>")),
                )?;
            }
        }

        writeln!(f, "\nNote: this is an internal error.")?;
        writeln!(f, "Help: please report this issue at https://github.com/TeamHarTex/HarTex/issues/new?assignees=&labels=Bot%3A+Bug%2CBot%3A+Internal+Error&template=internal-error.yml")?;
        write!(f, "\x1B[0m")
    }
}
