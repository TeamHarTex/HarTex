/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use hyper::{Body, Request};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[allow(clippy::module_name_repetitions)]
pub struct EventRequestActor {
    receiver: Receiver<Request<Body>>,
}

impl EventRequestActor {
    pub(self) fn new(receiver: Receiver<Request<Body>>) -> Self {
        Self { receiver }
    }

    pub(self) async fn run(&mut self) {
        while let Some(request) = self.receiver.recv().await {
            self.send_request(request).await;
        }
    }

    pub(self) async fn send_request(&self, _: Request<Body>) {
        todo!()
    }
}

pub struct EventRequestActorHandle {
    sender: Sender<Request<Body>>,
}

impl EventRequestActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(15);
        let mut actor = EventRequestActor::new(receiver);

        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }
}
