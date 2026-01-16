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

use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use dyn_clone::DynClone;
use ratatui::{
    Frame,
    layout::{Rect, Size},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{app::Action, tui::TuiEvent};

pub mod main;
pub mod pages;
pub mod tab_selector;

pub trait Component: DynClone {
    fn action_sender(&mut self, _: UnboundedSender<Action>) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()>;

    fn handle_event(&mut self, event: Option<TuiEvent>) -> Result<Option<Action>> {
        let action = match event {
            Some(TuiEvent::Key(key)) => self.handle_key_event(key)?,
            _ => None,
        };

        Ok(action)
    }

    fn handle_key_event(&mut self, _: KeyEvent) -> Result<Option<Action>>;

    fn initialize(&mut self, _: Size) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>>;
}

dyn_clone::clone_trait_object!(Component);
