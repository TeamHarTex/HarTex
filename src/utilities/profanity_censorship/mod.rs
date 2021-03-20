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
        HashMap
    },
    lazy::{
        SyncLazy
    }
};

static CHARACTER_ALIASES: SyncLazy<HashMap<char, char>> = SyncLazy::new(|| {
    let mut hashmap = HashMap::<char, char>::new();
    const CASE_DIFFERENCE: u8 = b'a' - b'A';

    for character in b'A'..=b'Z' {
        hashmap.insert(character as char, (character + CASE_DIFFERENCE) as char);
    }

    hashmap
});
