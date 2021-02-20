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

use twilight_model::{
    id::GuildId
};

use crate::{
    models::{
        levelling_system::Leaderboard
    },
    system::{
        twilight_http_client_extensions::{
            error::ClientExtensionResult,
            Pending
        },
    }
};

crate struct GetGuildLeaderboard {
    future: Option<Pending<Leaderboard>>,
    
    guild_id: GuildId
}
