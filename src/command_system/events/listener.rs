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
    sync::{
        Arc
    }
};

use dashmap::DashMap;

use futures_channel::mpsc::{
    UnboundedReceiver,
    UnboundedSender
};

/// Represents an event listener.
#[derive(Debug, Clone)]
crate struct Listener<T> {
    crate tx: UnboundedSender<T>
}

#[derive(Debug, Clone)]
struct ListenersRef<T> {
    id: u64,
    listeners: DashMap<u64, Listener<T>>
}

impl<T> Default for ListenersRef<T> {
    fn default() -> Self {
        Self {
            id: 0,
            listeners: DashMap::new(),
        }
    }
}

/// Represents a series of listeners in `DashMap`s.
#[derive(Debug, Clone)]
crate struct Listeners<T>(Arc<ListenersRef<T>>);

impl<T> Listeners<T> {
    crate fn add(&self) -> UnboundedReceiver<T> {
        let id = self.0.id + 1;
        let (tx, rx) = futures_channel::mpsc::unbounded();

        self.0.listeners.insert(id, Listener { tx });

        rx
    }

    crate fn all(&self) -> &DashMap<u64, Listener<T>> {
        &self.0.listeners
    }

    crate fn len(&self) -> usize {
        self.0.listeners.len()
    }
}

impl<T> Default for Listeners<T> {
    fn default() -> Self {
        Self(Arc::new(ListenersRef::default()))
    }
}
