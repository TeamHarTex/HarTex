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
    cmp::Ordering
};

use crate::{
    system::{
        twilight_id_extensions::IntoInnerU64
    }
};

mod entry;

crate use entry::LeaderboardEntry;

#[derive(Clone, Debug)]
crate struct Leaderboard {
    entries: Vec<LeaderboardEntry>
}

impl Leaderboard {
    crate fn new_with_vector(mut vector: Vec<LeaderboardEntry>) -> Self {
        vector.sort_unstable_by(|now, previous| {
            now.level.cmp(&previous.level)
                .then_with(||
                    now.experience.cmp(&previous.experience)
                )
                .then_with(||
                    now.user_id.into_inner_u64().cmp(&previous.user_id.into_inner_u64())
                )
        });
        
        vector.reverse();

        Self {
            entries: vector
        }
    }

    crate fn iter(&self) -> impl Iterator<Item = &LeaderboardEntry> + '_ {
        self.entries.iter()
    }
}
