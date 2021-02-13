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
    time::{
        Duration,
        SystemTime
    }
};

mod entry;

crate use entry::CachedEntry;

#[derive(Debug, Clone)]
crate struct SystemCache<K: Hash + Eq + Debug, V: Clone + Debug> {
    crate cache_table: HashMap<K, CachedEntry<V>>
}

impl<K: Hash + Eq + Debug, V: Clone + Debug> SystemCache<K, V> {
    crate fn new() -> Self {
        Self {
            cache_table: HashMap::new(),
        }
    }

    crate fn contains_key(&self, key: &K) -> bool {
        let now = SystemTime::now();

        self
            .cache_table
            .get(key)
            .filter(|entry| !entry.is_expired(now))
            .is_some()
    }

    crate fn get(&mut self, key: &K) -> Option<&V> {
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        self
            .cache_table
            .get(&key)
            .filter(|entry| !entry.is_expired(now))
            .map(|entry| &entry.value)
    }

    crate fn insert(&mut self, key: K, value: V, lifetime: Option<Duration>) -> Option<V> {
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        self
            .cache_table
            .insert(key, CachedEntry::new(value, lifetime))
            .filter(|entry| !entry.is_expired(now))
            .map(|entry| entry.value)
    }

    /*
    crate fn remove(&mut self, key: &K) -> Option<V> {
        let now = SystemTime::now();

        self.try_full_scan_expired_items(now);

        self
            .cache_table
            .remove(key)
            .filter(|entry| !entry.is_expired(now))
            .map(|entry| entry.value)
    }
     */

    fn try_full_scan_expired_items(&mut self, current_time: SystemTime) {
        self
            .cache_table
            .retain(|_, entry| !entry.is_expired(current_time));
    }
}
