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
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    sync::Arc
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

mod entry;

crate use entry::CachedEntry;

#[derive(Debug, Clone)]
crate struct SystemCache<K: Hash + Eq + Debug + Send + Clone, V: Clone + Debug + Send> {
    crate cache_table: HashMap<K, CachedEntry<V>>,

    crate background_task: Arc<Notify>
}

impl<K: Hash + Eq + Debug + Send + Clone + 'static, V: Clone + Debug + Send + 'static> SystemCache<K, V> {
    crate fn new() -> Self {
        let cache = Self {
            cache_table: HashMap::new(),

            background_task: Arc::new(Notify::new())
        };

        cache
    }

    crate fn contains_key(&self, key: &K) -> bool {
        self
            .cache_table
            .get(key)
            .is_some()
    }

    crate fn get(&mut self, key: &K) -> Option<&V> {
        self
            .cache_table
            .get(&key)
            .map(|entry| &entry.value)
    }

    crate fn insert(&mut self, key: K, value: V, lifetime: Option<Duration>) -> Option<V> {
        if let Some(lifetime) = lifetime {
            return self
                .cache_table
                .insert(key, CachedEntry::new(value, lifetime))
                .map(|entry| entry.value)
        }

        self.background_task.notified();

        None
    }

    crate fn purge_expired_items(&mut self) -> Option<Instant> {
        let current_time = Instant::now();

        while let Some((key, CachedEntry { expiration_time, .. })) = self.cache_table.clone().iter().next() {
            if *expiration_time > current_time {
                return Some(*expiration_time);
            }

            self.cache_table.remove(key);
        }

        None
    }
}
