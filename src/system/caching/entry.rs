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
};

use tokio::{
    time::{
        Duration,
        Instant
    }
};

#[derive(Clone, Debug)]
crate struct CachedEntry<V: Clone + Debug> {
    crate value: V,
    crate expiration_time: Instant
}

impl<V: Clone + Debug> CachedEntry<V> {
    crate fn new(value: V, lifetime: Duration) -> Self {
        Self {
            value,
            expiration_time: Instant::now() + lifetime
        }
    }

    crate fn is_expired(&self, current: Instant) -> bool {
        current >= self.expiration_time
    }
}
