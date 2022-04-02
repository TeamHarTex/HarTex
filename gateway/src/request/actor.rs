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

use hyper::client::Client;
use hyper::{Body, Request};
use tokio::sync::mpsc::{self, Receiver as MpscReceiver, Sender as MpscSender};

#[allow(clippy::module_name_repetitions)]
pub struct EventRequestActor {
    mpsc_receiver: MpscReceiver<EventRequest>,
}

pub struct EventRequest {
    request: Request<Body>,
    tries: u64,
}

impl EventRequest {
    pub fn new(request: Request<Body>) -> Self {
        Self { request, tries: 0 }
    }

    pub fn increment_tries(&mut self) {
        self.tries += 1;
    }
}

impl EventRequestActor {
    pub(self) fn new(
        mpsc_receiver: MpscReceiver<EventRequest>,
    ) -> Self {
        Self { mpsc_receiver }
    }

    pub(self) async fn run(&mut self) {
        while let Some(request) = self.mpsc_receiver.recv().await {
            self.send_request(request).await;
        }
    }

    pub(self) async fn send_request(&self, mut request: EventRequest) {
        log::trace!("sending request to request server");
        let client = Client::new();

        if let Err(error) = client.request(request.request).await {
            log::error!("failed to send request: {error}");
        }
    }
}

#[derive(Clone)]
pub struct EventRequestActorHandle {
    mpsc_sender: MpscSender<EventRequest>,
}

impl EventRequestActorHandle {
    pub fn new() -> Self {
        let (mpsc_sender, mpsc_receiver) = mpsc::channel(16);
        let mut actor = EventRequestActor::new(mpsc_receiver);

        tokio::spawn(async move { actor.run().await });

        Self { mpsc_sender }
    }

    pub async fn send(&self, request: Request<Body>) {
        self.mpsc_sender
            .send(EventRequest::new(request))
            .await
            .expect("failed to add request to queue");
    }
}
