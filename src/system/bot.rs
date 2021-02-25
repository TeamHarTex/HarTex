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

use twilight_cache_inmemory::{
    InMemoryCache
};

use twilight_gateway::{
    Cluster
};

use twilight_http::{
    Client as TwilightHttpClient
};

use crate::{
    command_system::{
        parser::CommandParser,
        CommandFramework
    },
    system::{
        caching::SystemCache,
        Stopwatch
    }
};

crate struct Bot<'a> {
    stopwatch: Stopwatch,
    cluster: Cluster,
    levelling_cache: SystemCache<String, bool>,
    http_client: TwilightHttpClient,
    framework: CommandFramework<'a>,
    parser: CommandParser<'a>,
    inmemory_cache: InMemoryCache
}

impl<'a> Bot<'a> {
    crate fn builder() -> BotBuilder<'a> {
        BotBuilder::new()
    }
}

crate struct BotBuilder<'a> {
    cluster: Option<Cluster>,
    http_client: Option<TwilightHttpClient>,
    framework: Option<CommandFramework<'a>>,
    parser: Option<CommandParser<'a>>,
    inmemory_cache: Option<InMemoryCache>
}

impl<'a> BotBuilder<'a> {
    fn new() -> Self {
        Self {
            cluster: None,
            http_client: None,
            framework: None,
            parser: None,
            inmemory_cache: None
        }
    }
}
