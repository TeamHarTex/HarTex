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
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, List, ListState},
};
use strum::{AsRefStr, Display, FromRepr, VariantNames};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::app::Action;

#[derive(Clone)]
pub struct TabSelector {
    action_tx: Option<UnboundedSender<Action>>,
    state: ListState,
}

impl TabSelector {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            action_tx: None,
            state,
        }
    }
}

impl Component for TabSelector {
    fn action_sender(&mut self, sender: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.action_tx.replace(sender);

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title_top(Line::from("Pages").centered());

        let list = List::new(Tab::VARIANTS.iter().copied())
            .block(block)
            .highlight_style(Style::new().black().on_white().bold());

        frame.render_stateful_widget(list, area, &mut self.state);

        Ok(())
    }

    fn handle_key_event(&mut self, _: KeyEvent) -> color_eyre::Result<Option<Action>> {
        Ok(None)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Next => {
                let Some(i) = self.state.selected_mut() else {
                    unreachable!()
                };

                *i = (*i + 1) % Tab::VARIANTS.len();
            }
            Action::Previous => {
                let Some(i) = self.state.selected_mut() else {
                    unreachable!()
                };

                *i = (*i - 1) % Tab::VARIANTS.len();
            }
            _ => return Ok(None),
        }

        self.action_tx
            .as_ref()
            .unwrap()
            .send(Action::SelectedPageChanged(
                self.state.selected().and_then(Tab::from_repr).unwrap(),
            ))?;

        Ok(None)
    }
}

#[derive(AsRefStr, Clone, Debug, Display, Eq, FromRepr, Hash, PartialEq, VariantNames)]
pub enum Tab {
    Overview,
    Shards,
}
