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

use crate::{
    std_extensions::IntegerDivRemSimultaneously
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
            _ => panic!("Entered unexpected branch of match in utilities/duration/mod.rs:45.")
        }
    }

    Duration::from_secs(dur)
}

crate fn seconds_to_ymwdhms(duration: u64) -> (u64, u64, u64, u64, u64, u64, u64) {
    let (minutes, seconds): (u64, u64) = duration.div_rem(&60);
    let (hours, minutes): (u64, u64) = minutes.div_rem(&60);
    let (days, hours): (u64, u64) = hours.div_rem(&24);
    let (weeks, days): (u64, u64) = days.div_rem(&7);

    (0, 0, weeks, days, hours, minutes, seconds)
}
