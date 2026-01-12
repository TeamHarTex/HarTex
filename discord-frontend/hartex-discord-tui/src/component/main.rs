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
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType},
};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, page::Page, tab_selector::TabSelector};
use crate::app::Action;

pub struct Main {
    page: Page,
    tab_selector: TabSelector,
}

impl Main {
    pub fn new() -> Self {
        Self {
            page: Page::new(),
            tab_selector: TabSelector::new(),
        }
    }
}

impl Component for Main {
    fn action_sender(&mut self, sender: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.tab_selector.action_sender(sender)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_top(Line::from("HarTex Management TUI").centered().bold())
            .title_bottom(Line::from(version()).centered().light_cyan());
        let inner = block.inner(area);

        frame.render_widget(block, area);

        let [left, right] = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(15), Constraint::Fill(1)],
        )
        .areas(inner);

        self.tab_selector.draw(frame, left)?;
        self.page.draw(frame, right)?;

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<Option<Action>> {
        self.tab_selector.handle_key_event(key)?;
        self.page.handle_key_event(key)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        self.tab_selector.update(action.clone())?;
        self.page.update(action)
    }
}
