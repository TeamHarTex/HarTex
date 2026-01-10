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
use hartex_version::version;
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType},
};

use super::Component;
use crate::app::Action;

pub struct Main;

impl Component for Main {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        frame.render_widget(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(Line::from("HarTex Management TUI").centered().bold())
                .title_bottom(Line::from(version()).centered().light_cyan()),
            area,
        );

        Ok(())
    }

    fn handle_key_event(&mut self, _: KeyEvent) -> color_eyre::Result<Option<Action>> {
        Ok(None)
    }

    fn update(&mut self, _: Action) -> color_eyre::Result<Option<Action>> {
        Ok(None)
    }
}
