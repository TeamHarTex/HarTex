//!  Copyright 2020 - 2021 The HarTex Project Developers
//!
//!  Licensed under the Apache License, Version 2.0 (the "License");
//!  you may not use this file except in compliance with the License.
//!  You may obtain a copy of the License at
//!
//!      http://www.apache.org/licenses/LICENSE-2.0
//!
//!  Unless required by applicable law or agreed to in writing, software
//!  distributed under the License is distributed on an "AS IS" BASIS,
//!  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!  See the License for the specific language governing permissions and
//!  limitations under the License.

use std::{
    collections::{
        HashSet
    },
    iter::{
        FromIterator
    },
    lazy::{
        SyncLazy
    }
};

crate static STAMDARD_WORD_SET: SyncLazy<HashSet<&str>> = SyncLazy::new(|| {
    HashSet::from_iter(vec![
        "ass",
        "asshole",
        "bitch",
        "cock",
        "cunt",
        "fag",
        "fagot",
        "faggot",
        "fuck",
        "nigger",
        "piss",
        "pussy",
        "shit",
        "twat",
        "whore"
    ])
});
