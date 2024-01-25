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

use console::style;
use console::Style;
use similar::ChangeTag;
use similar::TextDiff;

struct DiffLine(Option<usize>);

impl Display for DiffLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

pub fn compare_lines_and_render_if_needed(left: &str, right: &str) -> bool {
    let diff = TextDiff::from_lines(left, right);

    let mut to_print = String::new();

    let mut count = 0;
    for (i, group) in diff.grouped_ops(3).iter().enumerate() {
        if i > 0 {
            println!("{:-^1$}", "-", 80);
        }

        for op in group {
            diff.iter_inline_changes(op).for_each(|change| {
                let (sign, display) = match change.tag() {
                    ChangeTag::Delete => {
                        count += 1;
                        ("-", Style::new().red())
                    },
                    ChangeTag::Insert => {
                        count += 1;
                        ("+", Style::new().green())
                    },
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };

                to_print.push_str(&format!(
                    "{}{} |{}",
                    style(DiffLine(change.old_index())).dim(),
                    style(DiffLine(change.new_index())).dim(),
                    display.apply_to(sign).bold(),
                ));

                change.iter_strings_lossy().for_each(|(emphasized, value)| {
                    if emphasized {
                        to_print.push_str(&format!("{}", display.apply_to(value).underlined()));
                    } else {
                        to_print.push_str(&format!("{}", display.apply_to(value)));
                    }
                });
            });
        }
    }

    if count > 0 {
        println!("{to_print}");
        false
    } else {
        true
    }
}
