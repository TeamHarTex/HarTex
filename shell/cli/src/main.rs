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

use nu_ansi_term::{Color, Style};
use reedline::{DefaultHinter, FileBackedHistory, Reedline, Signal};

mod prompt;

pub fn main() {
    println!(
        "Welcome to HarTex Shell {}

Type `help` for a list of available commands.",
        env!("CARGO_PKG_VERSION")
    );

    let mut ed = Reedline::create()
        .with_ansi_colors(true)
        .with_hinter(Box::new(
            DefaultHinter::default().with_style(Style::new().fg(Color::DarkGray)),
        ))
        .with_history(Box::new(FileBackedHistory::new(5)));

    loop {
        match ed.read_line(&prompt::ShellPropmt) {
            Ok(Signal::Success(content)) => {
                shell_cmds::process_command(content);
            }
            Ok(Signal::CtrlC) => {
                return;
            }
            Err(_) => {}
            _ => (),
        }
    }
}
