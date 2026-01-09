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

pub use action::Action;
use color_eyre::eyre::Result;
use tokio::sync::{
    mpsc,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

use crate::{component::Component, tui::Tui};

mod action;

pub struct App {
    action_rx: UnboundedReceiver<Action>,
    action_tx: UnboundedSender<Action>,
    components: Vec<Box<dyn Component>>,
    fps: f64,
    quitting: bool,
    tps: f64,
}

impl App {
    pub fn new(fps: f64, tps: f64) -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        Self {
            action_rx,
            action_tx,
            components: Vec::new(),
            fps,
            quitting: false,
            tps,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?.fps(self.fps).tps(self.tps);
        tui.enter()?;

        for component in &mut self.components {
            component.action_sender(self.action_tx.clone())?;
            component.initialize(tui.size()?)?;
        }

        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;

            if self.quitting {
                tui.stop()?;
                break;
            }
        }

        tui.exit()?;
        Ok(())
    }

    fn handle_actions(&mut self, _: &mut Tui) -> Result<()> {
        while let Ok(_) = self.action_rx.try_recv() {
            todo!()
        }

        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };

        let action_tx = self.action_tx.clone();
        for component in &mut self.components {
            if let Some(action) = component.handle_event(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }

        Ok(())
    }
}
