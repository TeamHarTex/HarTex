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

use std::{
    io::{Stdout, stdout},
    ops::{Deref, DerefMut},
    time::Duration,
};

use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::EventStream,
    execute, terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
pub use event::TuiEvent;
use futures::{FutureExt, StreamExt};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::{
    sync::{
        mpsc,
        mpsc::{UnboundedReceiver, UnboundedSender},
    },
    task::JoinHandle,
    time::interval,
};
use tokio_util::sync::CancellationToken;

mod event;

pub struct Tui {
    cancellation_token: CancellationToken,
    event_rx: UnboundedReceiver<TuiEvent>,
    event_tx: UnboundedSender<TuiEvent>,
    fps: f64,
    task: JoinHandle<()>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    tps: f64,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();

        Ok(Self {
            cancellation_token: CancellationToken::new(),
            event_rx: rx,
            event_tx: tx,
            fps: 60.0,
            task: tokio::spawn(async {}),
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            tps: 4.0,
        })
    }
    
    pub fn fps(mut self, fps: f64) -> Self {
        self.fps = fps;
        self
    }
    
    pub fn tps(mut self, tps: f64) -> Self {
        self.tps = tps;
        self
    }

    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;

        execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
        self.start();

        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.stop()?;

        if terminal::is_raw_mode_enabled()? {
            self.flush()?;

            execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
            terminal::disable_raw_mode()?;
        }

        Ok(())
    }

    pub async fn next_event(&mut self) -> Option<TuiEvent> {
        self.event_rx.recv().await
    }

    pub fn start(&mut self) {
        self.cancel();
        self.cancellation_token = CancellationToken::new();

        let event_loop = Self::event_loop(
            self.cancellation_token.clone(),
            self.event_tx.clone(),
            self.fps,
            self.tps,
        );
        self.task = tokio::spawn(async {
            event_loop.await;
        });
    }

    pub fn stop(&mut self) -> Result<()> {
        self.cancel();
        Ok(())
    }

    async fn event_loop(
        cancellation_token: CancellationToken,
        event_tx: UnboundedSender<TuiEvent>,
        fps: f64,
        tps: f64,
    ) {
        let mut event_stream = EventStream::new();
        let mut fps_interval = interval(Duration::from_secs_f64(1.0 / fps));
        let mut tps_interval = interval(Duration::from_secs_f64(1.0 / tps));

        event_tx
            .send(TuiEvent::Initialized)
            .expect("failed to send initialized event");

        loop {
            let event = tokio::select! {
                _ = cancellation_token.cancelled() => break,
                _ = fps_interval.tick() => TuiEvent::Render,
                _ = tps_interval.tick() => TuiEvent::Tick,
                ct_event = event_stream.next().fuse() => match ct_event {
                    _ => continue,
                }
            };

            if event_tx.send(event).is_err() {
                break;
            }
        }

        cancellation_token.cancel();
    }
}

impl Deref for Tui {
    type Target = Terminal<CrosstermBackend<Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}
