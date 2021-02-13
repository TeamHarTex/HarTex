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
    fmt::Debug,
    time::{
        Duration,
        SystemTime
    }
};

#[derive(Clone, Debug)]
crate struct CachedEntry<V: Clone + Debug> {
    crate value: V,

    expiration_time: Option<SystemTime>
}

impl<V: Clone + Debug> CachedEntry<V> {
    crate fn new(value: V, lifetime: Option<Duration>) -> Self {
        Self {
            value,

            expiration_time: lifetime.map(|duration| SystemTime::now() + duration)
        }
    }

    crate fn is_expired(&self, current: SystemTime) -> bool {
        self
            .expiration_time
            .map_or(false, |time| current >= time)
    }
}
