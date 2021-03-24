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

crate static STANDARD_WORD_SET: SyncLazy<HashSet<String>> = SyncLazy::new(|| {
    HashSet::from_iter(vec![
        "ass".to_string(),
        "asshole".to_string(),
        "bitch".to_string(),
        "cock".to_string(),
        "cunt".to_string(),
        "fag".to_string(),
        "fagot".to_string(),
        "faggot".to_string(),
        "fuck".to_string(),
        "nigger".to_string(),
        "piss".to_string(),
        "pussy".to_string(),
        "shit".to_string(),
        "twat".to_string(),
        "whore".to_string()
    ])
});

crate static ZEALOUS_WORD_SET: SyncLazy<HashSet<String>> = SyncLazy::new(|| {
    HashSet::from_iter(vec![
        "crap".to_string(),
        "damn".to_string(),
        "goddamn".to_string(),
        "hell".to_string(),
        "suck".to_string()
    ])
});

crate static SEX_WORD_SET: SyncLazy<HashSet<String>> = SyncLazy::new(|| {
    HashSet::from_iter(vec![
        "ass".to_string(),
        "asshole".to_string(),
        "blowjob".to_string(),
        "boob".to_string(),
        "boobie".to_string(),
        "boobies".to_string(),
        "boobjob".to_string(),
        "breast".to_string(),
        "clitoris".to_string(),
        "cock".to_string(),
        "condom".to_string(),
        "cunnilingus".to_string(),
        "cunt".to_string(),
        "dick".to_string(),
        "doggystyle".to_string(),
        "ejaculate".to_string(),
        "felate".to_string(),
        "felatio".to_string(),
        "fetish".to_string(),
        "foreskin".to_string(),
        "handjob".to_string(),
        "labia".to_string(),
        "masturbate".to_string(),
        "masturbation".to_string(),
        "masterbate".to_string(),
        "masterbation".to_string(),
        "penis".to_string(),
        "pussy".to_string(),
        "rimjob".to_string(),
        "semen".to_string(),
        "sex".to_string(),
        "tits".to_string(),
        "tittie".to_string(),
        "titties".to_string(),
        "titty".to_string(),
        "twat".to_string(),
        "vagina".to_string(),
        "vulva".to_string()
    ])
});
