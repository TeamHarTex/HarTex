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
    time::Duration
};

crate fn parse_duration(duration: String) -> Duration {
    let mut acc = 0u64;
    let mut dur = 0u64;

    for c in duration.chars() {
        match c {
            | '0'..'9' => {
                acc *= 10;
                acc += c.to_digit(10).unwrap() as u64;
            },
            | 'd' | 'D' => {
                dur += acc * 24 * 60 * 60;
                acc = 0;
            },
            | 'm' | 'M' => {
                dur += acc * 60;
                acc = 0;
            },
            | 'h' | 'H' => {
                dur += acc * 60 * 60;
                acc = 0;
            },
            | 's' | 'S' => {
                dur += acc * 60;
                acc = 0;
            },
            _ => unreachable!(),
        }
    }

    Duration::from_secs(dur)
}
