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
    lazy::{
        SyncLazy
    }
};

crate mod character_aliases;
crate mod word_sets;

crate enum ProfanityCensorshipOptions {
    UseStandardWordSetOnly,

    UseSexWordSetOnly,

    UseZealousWordSetOnly,

    UseCustomWordSet(HashSet<String>)
}

impl ProfanityCensorshipOptions {
    crate fn empty() -> Self {
        Self::UseCustomWordSet(HashSet::new())
    }

    crate fn custom<I, T>(words: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String> {
        Self::UseCustomWordSet(words.into_iter().map(Into::into).collect())
    }
}

impl Default for ProfanityCensorshipOptions {
    fn default() -> Self {
        Self::UseStandardWordSetOnly
    }
}

fn alias(text: &str) -> String {
    text.chars()
        .map(|character| character_aliases::CHARACTER_ALIASES.get(&character).copied().unwrap_or(character))
        .collect()
}
