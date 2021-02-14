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
};

use tokio::{
    time::{
        Duration,
        Instant
    }
};

mod entry;

crate use entry::CachedEntry;

#[derive(Debug, Clone)]
crate struct SystemCache<K: Hash + Eq + Debug + Clone + Send, V: Clone + Debug + Send> {
    crate cache_table: HashMap<K, CachedEntry<V>>
}

impl<K: Hash + Eq + Debug + Clone + Send + 'static, V: Clone + Debug + Send + 'static> SystemCache<K, V> {
    crate fn new() -> Self {
        let cache = Self {
            cache_table: HashMap::new(),
        };

        tokio::spawn(purge_expired_background_task(cache.clone()));

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

async fn purge_expired_background_task<K: Hash + Eq + Debug + Clone + Send + 'static, V: Clone + Debug + Send + 'static>(mut cache: SystemCache<K, V>) {
    loop {
        if let Some(when) = cache.purge_expired_items() {
            tokio::join!(tokio::time::sleep_until(when));
        }
    }
}
