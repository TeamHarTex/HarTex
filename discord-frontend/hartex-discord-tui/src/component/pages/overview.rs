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

use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
};

use crate::{app::Action, component::Component, widgets::overview_table::OverviewTable};

#[derive(Clone)]
pub struct OverviewPage;

impl Component for OverviewPage {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) -> color_eyre::Result<()> {
        let centering = rect.centered(Constraint::Percentage(75), Constraint::Percentage(75));
        frame.render_widget(OverviewTable::new("Test"), centering);
        Ok(())
    }

    fn handle_key_event(&mut self, _: KeyEvent) -> color_eyre::Result<Option<Action>> {
        Ok(None)
    }

    fn update(&mut self, _: Action) -> color_eyre::Result<Option<Action>> {
        Ok(None)
    }
}
