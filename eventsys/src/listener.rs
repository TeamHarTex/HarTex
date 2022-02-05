/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
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

//! Listeners for sending events.

use std::sync::Arc;

use dashmap::DashMap;
use futures_channel::mpsc::{
    self,
    UnboundedReceiver,
    UnboundedSender
};

/// An event listener.
#[derive(Debug, Clone)]
pub struct Listener<T> {
    pub sender: UnboundedSender<T>
}

/// A series of `Listener`s.
#[derive(Debug, Clone)]
pub struct Listeners<T> {
    inner: Arc<ListenersInner<T>>
}

impl<T> Default for Listeners<T> {
    fn default() -> Self {
        Self {
            inner: Arc::new(ListenersInner::default())
        }
    }
}

impl<T> Listeners<T> {
    /// Creates a new listener.
    #[must_use]
    pub fn add(&self) -> UnboundedReceiver<T> {
        let id = self.inner.id + 1;
        let (sender, receiver) = mpsc::unbounded();

        self.inner.listeners.insert(
            id,
            Listener {
                sender
            }
        );

        receiver
    }

    /// Returns the total number of listeners present.
    #[allow(clippy::len_without_is_empty)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.listeners.len()
    }

    /// Returns all the listeners.
    #[must_use]
    pub fn listeners(&self) -> &DashMap<u64, Listener<T>> {
        &self.inner.listeners
    }
}

#[derive(Debug)]
struct ListenersInner<T> {
    id: u64,
    listeners: DashMap<u64, Listener<T>>
}

impl<T> Default for ListenersInner<T> {
    fn default() -> Self {
        Self {
            id: 0,
            listeners: DashMap::default()
        }
    }
}
