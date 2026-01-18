/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Widget,
};

pub struct OverviewTable {
    username: String,
    user_id: String,
}

impl OverviewTable {
    pub fn new(username: impl Into<String>, user_id: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            user_id: user_id.into(),
        }
    }
}

impl Widget for OverviewTable {
    fn render(self, rect: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let y = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]);
        let x = Layout::horizontal([
            Constraint::Percentage(29),
            Constraint::Percentage(69),
        ]).spacing(2);

        let grid = rect
            .layout_vec(&y)
            .iter()
            .map(|row| row.layout_vec(&x))
            .collect::<Vec<_>>();

        let username_left = grid[0][0];
        let username_right = grid[0][1];

        Line::raw("Username:")
            .bold()
            .right_aligned()
            .render(username_left, buf);
        Line::raw(self.username)
            .light_cyan()
            .render(username_right, buf);

        let id_left = grid[1][0];
        let id_right = grid[1][1];

        Line::raw("User ID:").bold().right_aligned().render(id_left, buf);
        Line::raw(self.user_id).light_cyan().render(id_right, buf);
    }
}
