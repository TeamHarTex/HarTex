///  Copyright 2020 - 2021 The HarTex Project Developers
///
///  Licensed under the Apache License, Version 2.0 (the "License");
///  you may not use this file except in compliance with the License.
///  You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
///  Unless required by applicable law or agreed to in writing, software
///  distributed under the License is distributed on an "AS IS" BASIS,
///  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
///  See the License for the specific language governing permissions and
///  limitations under the License.

use std::{
    pin::Pin,
    task::{
        Context,
        Poll
    }
};

use futures_channel::mpsc::UnboundedReceiver;
use futures_util::{
    stream::Stream,
    StreamExt
};

use crate::system::model::payload::{
    CommandExecuted,
    CommandFailed,
    CommandReceived
};

/// Represents some events.
crate struct CommandEvents {
    rx: UnboundedReceiver<SystemEvent>
}

impl<'a> CommandEvents {
    crate fn new(rx: UnboundedReceiver<SystemEvent>) -> Self {
        Self {
            rx
        }
    }
}

impl Stream for CommandEvents {
    type Item = SystemEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

/// The variants of events that can be emitted.
#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
crate enum SystemEvent {
    CommandReceived(Box<CommandReceived>),
    CommandIdentified(String),
    CommandExecuted(Box<CommandExecuted>),
    CommandFailed(Box<CommandFailed>),
}
