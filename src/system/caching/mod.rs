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
    collections::{
        BTreeMap,
        HashMap
    },
    fmt::Debug,
    hash::Hash,
    sync::{
        Arc,
        Mutex
    }
};

use tokio::{
    sync::{
        Notify
    },
    time::{
        Duration,
        Instant
    }
};

#[derive(Debug, Clone)]
crate struct SystemCache<K: Hash + Eq + Debug + Send + Clone, V: Clone + Debug + Send> {
    shared: Arc<SharedInternal<K, V>>
}

#[derive(Debug)]
struct SharedInternal<K: Hash + Eq + Debug + Send + Clone, V: Clone + Debug + Send> {
    state: Mutex<StateInternal<K, V>>,
    background_task: Notify
}

#[derive(Debug)]
struct StateInternal<K: Hash + Eq + Debug + Send + Clone, V: Clone + Debug + Send> {
    entries: HashMap<K, SystemCacheEntry<V>>,
    expirations: BTreeMap<(Instant, u64), K>,
    next_id: u64
}

#[derive(Debug)]
struct SystemCacheEntry<V: Clone + Debug + Send> {
    entry_id: u64,
    data: V,
    expires_at: Option<Instant>
}

impl<K: Hash + Eq + Debug + Send + Clone + 'static, V: Clone + Debug + Send + 'static> SystemCache<K, V> {
    crate fn new() -> Self {
        let shared = Arc::new(
            SharedInternal {
                state: Mutex::new(
                    StateInternal {
                        entries: HashMap::new(),
                        expirations: BTreeMap::new(),
                        next_id: 0
                    }
                ),
                background_task: Notify::new()
            }
        );

        tokio::spawn(purged_expired_entries_task(shared.clone()));

        Self {
            shared
        }
    }

    crate fn get(&self, key: &K) -> Option<V> {
        let state = self.shared.state.lock().unwrap();

        state.entries.get(key).map(|entry| entry.data.clone())
    }

    crate fn insert(&self, key: K, value: V, expire: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();

        let id = state.next_id;
        state.next_id += 1;

        let mut notify = false;

        let expire_at = expire.map(|duration| {
            let when = Instant::now() + duration;

            notify = state
                .next_expiration()
                .map(|expiration| expiration > when)
                .unwrap_or(true);

            state.expirations.insert((when, id), key.clone());

            when
        });

        let previous_key = state.entries.insert(
            key,
            SystemCacheEntry {
                entry_id: id,
                data: value,
                expires_at: expire_at
            }
        );

        if let Some(previous) = previous_key {
            if let Some(when) = previous.expires_at {
                state.expirations.remove(&(when, previous.entry_id));
            }
        }

        drop(state);

        if notify {
            self.shared.background_task.notify_one();
        }
    }
}

impl<K: Hash + Eq + Debug + Send + Clone, V: Clone + Debug + Send> SharedInternal<K, V> {
    fn purge_expired_entries(&self) -> Option<Instant> {
        let mut state = self.state.lock().unwrap();

        let state = &mut *state;
        let now = Instant::now();

        while let Some((&(when, id), key)) = state.expirations.iter().next() {
            if when > now {
                return Some(when);
            }

            state.entries.remove(key);
            state.expirations.remove(&(when, id));
        }

        None
    }
}

impl<K: Hash + Eq + Debug + Send + Clone, V: Clone + Debug + Send> StateInternal<K, V> {
    fn next_expiration(&self) -> Option<Instant> {
        self
            .expirations
            .keys()
            .next()
            .map(|entry| entry.0)
    }
}

async fn purged_expired_entries_task<K: Hash + Eq + Debug + Send + Clone, V: Clone + Debug + Send>(shared: Arc<SharedInternal<K, V>>) {
    loop {
        if let Some(when) = shared.purge_expired_entries() {
            tokio::select! {
                _ = tokio::time::sleep_until(when) => {},
                _ = shared.background_task.notified() => {}
            }
        }
        else {
            shared.background_task.notified().await;
        }
    }
}
