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

use hartex_discord_grpc::manager::WorkerSessionStartLimit;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::Widget,
};
use time::{format_description::well_known::Rfc2822, Duration, OffsetDateTime};

pub struct OverviewTable {
    username: String,
    user_id: String,
    limits: Option<WorkerSessionStartLimit>,
    shards: Option<u32>,
}

impl OverviewTable {
    pub fn new(
        username: impl Into<String>,
        user_id: impl Into<String>,
        limits: Option<WorkerSessionStartLimit>,
        shards: Option<u32>,
    ) -> Self {
        Self {
            username: username.into(),
            user_id: user_id.into(),
            limits,
            shards,
        }
    }
}

impl Widget for OverviewTable {
    fn render(self, rect: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let y = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]);
        let x =
            Layout::horizontal([Constraint::Percentage(29), Constraint::Percentage(69)]).spacing(2);

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

        Line::raw("User ID:")
            .bold()
            .right_aligned()
            .render(id_left, buf);
        Line::raw(self.user_id).light_cyan().render(id_right, buf);

        let Some(limits) = self.limits else {
            return;
        };
        let Some(shards) = self.shards else {
            return;
        };

        let recommended_left = grid[2][0];
        let recommended_right = grid[2][1];

        Line::raw("Recommended shards:")
            .bold()
            .right_aligned()
            .render(recommended_left, buf);
        Line::raw(shards.to_string())
            .light_cyan()
            .render(recommended_right, buf);

        let available_left = grid[3][0];
        let available_right = grid[3][1];

        Line::raw("Sessions available:")
            .bold()
            .right_aligned()
            .render(available_left, buf);
        Line::raw(format!(
            "{}/{} ({:.2}%)",
            limits.remaining,
            limits.total,
            (f64::from(limits.remaining) / f64::from(limits.total)) * 100_f64
        ))
        .light_cyan()
        .render(available_right, buf);

        let reset_after_left = grid[4][0];
        // let reset_after_right = grid[4][1];

        Line::raw("Session resets at:")
            .bold()
            .right_aligned()
            .render(reset_after_left, buf);
        // Line::raw(
        //     (OffsetDateTime::now_local().unwrap() + Duration::milliseconds(limits.reset_after as i64))
        //         .format(&Rfc2822)
        //         .unwrap(),
        // )
        // .light_cyan()
        // .render(reset_after_right, buf);

        let concurrency_left = grid[5][0];
        let concurrency_right = grid[5][1];
        Line::raw("Max concurrency:")
            .bold()
            .right_aligned()
            .render(concurrency_left, buf);
        Line::raw(limits.max_concurrency.to_string())
            .light_cyan()
            .render(concurrency_right, buf);
    }
}
