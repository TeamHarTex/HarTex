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
    convert::TryInto,
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    }
};

use chrono::{
    DateTime,
    Local
};

use twilight_model::{
    gateway::{
        presence::{
            activity_button::ActivityButtonLink,
            Activity,
            ActivityButton,
            ActivityType,
        },
    }
};

crate mod bot_configuration;
crate mod caching;
crate mod event_handler;
crate mod model;
crate mod panicking;
crate mod state_machine;
crate mod terminal;
crate mod twilight_http_client_extensions;
crate mod twilight_id_extensions;
crate mod whitelisting;

crate type SystemResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

crate enum EventType {
    TwilightEvent,
    CustomEvent
}

#[derive(Debug)]
crate struct SystemError(crate String);

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Error for SystemError {}

#[derive(Copy, Clone)]
crate struct Stopwatch {
    start: DateTime<Local>
}

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch {
            start: Local::now()
        }
    }

    #[allow(dead_code)]
    pub fn elapsed_seconds(&self) -> u128 {
        let now = Local::now();

        (now - self.start).num_seconds().try_into().unwrap()
    }

    pub fn elapsed_milliseconds(&self) -> u128 {
        let now = Local::now();

        (now - self.start).num_milliseconds().try_into().unwrap()
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

crate fn set_bot_activity() -> Activity {
    Activity {
        application_id: None,
        assets: None,
        buttons: vec![
            ActivityButton::Link(ActivityButtonLink {
                label: String::from("GitHub Source Code Repository"),
                url: String::from("https://github.com/HT-Studios/HarTex-rust-discord-bot")
            })
        ],
        created_at: None,
        details: None,
        emoji: None,
        flags: None,
        id: None,
        instance: None,
        kind: ActivityType::Watching,
        name: String::from("Being developed & stabilized"),
        party: None,
        secrets: None,
        state: None,
        timestamps: None,
        url: None
    }
}
