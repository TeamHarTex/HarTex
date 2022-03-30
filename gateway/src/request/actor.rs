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

use std::time::Duration;

use hyper::client::Client;
use hyper::{Body, Request};
use hyper::header::AUTHORIZATION;
use tokio::sync::mpsc::{self, Receiver as MpscReceiver, Sender as MpscSender};
use tokio::time;

#[allow(clippy::module_name_repetitions)]
pub struct EventRequestActor {
    mpsc_receiver: MpscReceiver<(Request<Body>, usize)>,
    mpsc_sender: MpscSender<(Request<Body>, usize)>,
}

impl EventRequestActor {
    pub(self) fn new(
        mpsc_receiver: MpscReceiver<(Request<Body>, usize)>,
    ) -> (Self, MpscReceiver<(Request<Body>, usize)>) {
        let (mpsc_sender, new_mpsc_receiver) = mpsc::channel(16);

        (
            Self {
                mpsc_receiver,
                mpsc_sender,
            },
            new_mpsc_receiver,
        )
    }

    pub(self) async fn run(&mut self) {
        while let Some((request, tries)) = self.mpsc_receiver.recv().await {
            self.send_request(request, tries).await;
        }
    }

    pub(self) async fn send_request(&self, request: Request<Body>, tries: usize) {
        log::trace!("sending request to request server");
        let client = Client::new();

        if let Err(error) = client.request(request).await {
            log::error!("failed to send request: {error}");

            if let Err(error) = self.mpsc_sender.send((request, tries + 1)).await {
                log::error!("failed to send failed request back to handle: {error}");
            }
        }
    }
}

pub struct EventRequestActorHandle {
    mpsc_receiver: MpscReceiver<(Request<Body>, usize)>,
    mpsc_sender: MpscSender<(Request<Body>, usize)>,
}

impl EventRequestActorHandle {
    pub fn new() -> Self {
        let (mpsc_sender, mpsc_receiver) = mpsc::channel(16);
        let (mut actor, new_mpsc_receiver) = EventRequestActor::new(mpsc_receiver);

        tokio::spawn(async move { actor.run().await });

        let mut this = Self {
            mpsc_receiver: new_mpsc_receiver,
            mpsc_sender,
        };
        tokio::spawn(this.run());

        this
    }

    pub(self) async fn run(&mut self) {
        while let Some((failed_request, tries)) = self.mpsc_receiver.recv().await {
            log::trace!("received a failed request to endpoint {}, tries: {tries}", failed_request.uri());
            log::trace!("sleeping for {} seconds before retrying", tries * 5);

            time::sleep(Duration::from_secs((tries * 5) as u64));

            log::trace!("retrying request...");
            self.mpsc_sender.send((failed_request, tries));
        }
    }

    pub async fn send(&self, request: Request<Body>) {
        self.mpsc_sender
            .send((request, 0))
            .await
            .expect("failed to add request to queue");
    }
}
