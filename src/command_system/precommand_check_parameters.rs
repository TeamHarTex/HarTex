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

use twilight_cache_inmemory::{
    InMemoryCache
};

use twilight_model::{
    id::{
        UserId,
        GuildId,
        RoleId
    }
};

#[derive(Clone)]
crate struct PrecommandCheckParameters {
    pub user_id: Option<UserId>,
    pub guild_id: Option<GuildId>,
    pub role_id: Option<RoleId>,
    pub cache: Option<InMemoryCache>,
    pub minimum_permission_level: Option<u32>
}

impl PrecommandCheckParameters {
    #[allow(dead_code)]
    crate fn builder() -> PrecommandCheckParametersBuilder {
        PrecommandCheckParametersBuilder::new()
    }
}

#[non_exhaustive]
crate struct PrecommandCheckParametersBuilder {
    pub user_id: Option<UserId>,
    pub guild_id: Option<GuildId>,
    pub role_id: Option<RoleId>,
    pub cache: Option<InMemoryCache>,
    pub minimum_permission_level: Option<u32>
}

impl PrecommandCheckParametersBuilder {
    crate fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    crate fn user_id(mut self, user_id: UserId) -> Self {
        self.user_id.replace(user_id);

        self
    }

    crate fn guild_id(mut self, guild_id: GuildId) -> Self {
        self.guild_id.replace(guild_id);

        self
    }

    crate fn in_memory_cache(mut self, cache: InMemoryCache) -> Self {
        self.cache.replace(cache);

        self
    }

    crate fn minimum_permission_level(mut self, permission_level: u32) -> Self {
        self.minimum_permission_level.replace(permission_level);

        self
    }

    crate fn build(self) -> PrecommandCheckParameters {
        PrecommandCheckParameters {
            user_id: self.user_id,
            guild_id: self.guild_id,
            role_id: self.role_id,
            cache: self.cache,
            minimum_permission_level: self.minimum_permission_level
        }
    }
}

impl Default for PrecommandCheckParametersBuilder {
    fn default() -> Self {
        Self {
            user_id: None,
            guild_id: None,
            role_id: None,
            cache: None,
            minimum_permission_level: None
        }
    }
}
