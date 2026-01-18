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

use std::collections::HashMap;

use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use hartex_discord_grpc::manager::{WhoamiRequest, manager_client::ManagerClient};
use ratatui::layout::Rect;
use tokio::sync::{
    mpsc,
    mpsc::{UnboundedReceiver, UnboundedSender},
};
use tonic::transport::Channel;

pub use self::{action::Action, menu::Menu};
use crate::{
    component::{Component, main::Main},
    lazies::KEYBINDS,
    tui::{Tui, TuiEvent},
};

mod action;
mod menu;

pub struct App {
    action_rx: UnboundedReceiver<Action>,
    action_tx: UnboundedSender<Action>,
    client: ManagerClient<Channel>,
    components: Vec<Box<dyn Component>>,
    fps: f64,
    keybinds: HashMap<Menu, HashMap<Vec<KeyEvent>, Action>>,
    last_tick_key_events: Vec<KeyEvent>,
    menu: Menu,
    quitting: bool,
    tps: f64,
}

impl App {
    pub fn new(fps: f64, tps: f64, client: ManagerClient<Channel>) -> Self {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        Self {
            action_rx,
            action_tx,
            client,
            components: vec![Box::new(Main::new())],
            fps,
            keybinds: KEYBINDS.clone(),
            last_tick_key_events: Vec::new(),
            menu: Menu::Main,
            quitting: false,
            tps,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let whoami = self
            .client
            .whoami(WhoamiRequest::default())
            .await?
            .into_inner();
        let username = format!("{}#{}", whoami.username, whoami.discriminator);

        let mut tui = Tui::new()?.fps(self.fps).tps(self.tps);
        tui.enter()?;

        for component in &mut self.components {
            component.action_sender(self.action_tx.clone())?;
            component.initialize(tui.size()?)?;
        }

        self.action_tx.send(Action::Whoami {
            username,
            user_id: whoami.user_id,
        })?;

        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;

            if self.quitting {
                tui.stop();
                break;
            }
        }

        tui.exit()?;
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::Quit => self.quitting = true,
                Action::Resize(w, h) => self.resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                _ => {}
            }

            for component in &mut self.components {
                if let Some(action) = component.update(action.clone())? {
                    self.action_tx.send(action)?;
                }
            }
        }

        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };

        let action_tx = self.action_tx.clone();
        match event {
            TuiEvent::Initialized => {}
            TuiEvent::Key(key) => self.handle_key_event(key)?,
            TuiEvent::Resize(w, h) => action_tx.send(Action::Resize(w, h))?,
            TuiEvent::Render => action_tx.send(Action::Render)?,
            TuiEvent::Tick => action_tx.send(Action::Tick)?,
        }

        for component in &mut self.components {
            if let Some(action) = component.handle_event(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.keybinds.get(&self.menu) else {
            return Ok(());
        };

        let Some(action) = keymap.get(&vec![event]).or_else(|| {
            self.last_tick_key_events.push(event);
            keymap.get(&self.last_tick_key_events)
        }) else {
            return Ok(());
        };

        action_tx.send(action.clone())?;

        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            for component in &mut self.components {
                if let Err(e) = component.draw(frame, frame.area()) {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("failed to draw component: {e}")));
                }
            }
        })?;

        Ok(())
    }

    fn resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;

        Ok(())
    }
}
