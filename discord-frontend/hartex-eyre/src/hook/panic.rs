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
use std::thread;

use backtrace::Backtrace;

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
            hook: self,
            panic_info,
        }
    }
}

pub struct PanicReport<'a> {
    backtrace: Backtrace,
    hook: &'a PanicHook,
    panic_info: &'a PanicInfo<'a>,
}

impl Display for PanicReport<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "\r\x1B[2K")?;

        writeln!(
            f,
            "\x1B[0;31mPanic occurred in thread {:?}: {}",
            thread::current().name().unwrap_or("<unknown>"),
            self.panic_info.payload().downcast_ref::<&str>().unwrap_or(&"Box<dyn Any>")
        )?;

        write!(f, "\x1B[0m")
    }
}
