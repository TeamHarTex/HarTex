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

use std::{
    error::Error,
    future::Future,
    pin::Pin
};

use twilight_http::{
    Client
};

use twilight_model::{
    id::{
        GuildId,
        UserId
    }
};

use crate::system::model::{
    infraction_update_type::InfractionUpdateType,
    infractions::InfractionType
};

use database_manipulation::{
    AddUserInfraction as AddUserInfractionStruct,
    ClearUserInfractions as ClearUserInfractionsStruct,
    GetGuildConfiguration as GuildConfigurationStruct,
    GetGuildInfractions as GetGuildInfractionsStruct,
    GetLocalUserInfractions as GetLocalUserInfractionsStruct,
    GetWhitelistedGuilds as WhitelistedGuildsStruct,
    InitializeWhitelistedGuild as InitializeWhitelistedGuildStruct,
    RemoveUserInfraction as RemoveUserInfractionStruct,
    UpdateUserInfraction as UpdateUserInfractionStruct
};

use levelling_system::{
    AddUserExperience as AddUserExperienceStruct,
    GetGuildLeaderboard as GetGuildLeaderboardStruct,
    GetUserExperience as GetUserExperienceStruct
};

pub mod database_manipulation;
pub mod levelling_system;
pub mod error;

type Pending<T> = Pin<Box<dyn Future<Output = Result<T, Box<dyn Error + Send + Sync>>>>>;

// Section: Infraction System

crate trait AddUserInfraction {
    fn add_user_infraction(self, warning_id: String, guild_id: GuildId, user_id: UserId, reason: String, infraction_type: InfractionType) -> AddUserInfractionStruct;
}

crate trait GetWhitelistedGuilds {
    fn get_whitelisted_guilds(self) -> WhitelistedGuildsStruct;
}

crate trait ClearUserInfractions {
    fn clear_user_infractions(self, guild_id: GuildId, user_id: UserId) -> ClearUserInfractionsStruct;
}

crate trait GetGuildConfiguration {
    fn get_guild_configuration(self, guild_id: GuildId) -> GuildConfigurationStruct;
}

crate trait GetGuildInfractions {
    fn get_guild_infractions(self, guild_id: GuildId) -> GetGuildInfractionsStruct;
}

crate trait GetLocalUserInfractions {
    fn get_local_user_infractions(self, guild_id: GuildId, user_id: UserId) -> GetLocalUserInfractionsStruct;
}

crate trait InitializeWhitelistedGuild {
    fn initialize_whitelisted_guild(self, guild_id: GuildId) -> InitializeWhitelistedGuildStruct;
}

crate trait RemoveUserInfraction {
    fn remove_user_infraction(self, guild_id: GuildId, user_id: UserId, infraction_id: String) -> RemoveUserInfractionStruct;
}

crate trait UpdateUserInfraction {
    fn update_user_infraction(self, infraction_id: String, guild_id: GuildId, user_id: UserId, update_type: InfractionUpdateType) -> UpdateUserInfractionStruct;
}

impl AddUserInfraction for Client {
    fn add_user_infraction(self, infraction_id: String, guild_id: GuildId, user_id: UserId, reason: String, infraction_type: InfractionType) -> AddUserInfractionStruct {
        AddUserInfractionStruct::new(infraction_id, guild_id, user_id, reason, infraction_type)
    }
}

impl ClearUserInfractions for Client {
    fn clear_user_infractions(self, guild_id: GuildId, user_id: UserId) -> ClearUserInfractionsStruct {
        ClearUserInfractionsStruct::new(guild_id, user_id)
    }
}

impl GetWhitelistedGuilds for Client {
    fn get_whitelisted_guilds(self) -> WhitelistedGuildsStruct {
        WhitelistedGuildsStruct::new()
    }
}

impl GetGuildConfiguration for Client {
    fn get_guild_configuration(self, guild_id: GuildId) -> GuildConfigurationStruct {
        GuildConfigurationStruct::new(guild_id)
    }
}

impl GetGuildInfractions for Client {
    fn get_guild_infractions(self, guild_id: GuildId) -> GetGuildInfractionsStruct {
        GetGuildInfractionsStruct::new(guild_id, self)
    }
}

impl GetLocalUserInfractions for Client {
    fn get_local_user_infractions(self, guild_id: GuildId, user_id: UserId) -> GetLocalUserInfractionsStruct {
        GetLocalUserInfractionsStruct::new(guild_id, user_id)
    }
}

impl InitializeWhitelistedGuild for Client {
    fn initialize_whitelisted_guild(self, guild_id: GuildId) -> InitializeWhitelistedGuildStruct {
        InitializeWhitelistedGuildStruct::new(self, guild_id)
    }
}

impl RemoveUserInfraction for Client {
    fn remove_user_infraction(self, guild_id: GuildId, user_id: UserId, infraction_id: String) -> RemoveUserInfractionStruct {
        RemoveUserInfractionStruct::new(infraction_id, guild_id, user_id)
    }
}

impl UpdateUserInfraction for Client {
    fn update_user_infraction(self, infraction_id: String, guild_id: GuildId, user_id: UserId, update_type: InfractionUpdateType) -> UpdateUserInfractionStruct {
        UpdateUserInfractionStruct::new(infraction_id, guild_id, user_id, update_type)
    }
}

// Section: Levelling System

crate trait AddUserExperience {
    fn add_user_experience(self, guild_id: GuildId, user_id: UserId, experience: u64) -> AddUserExperienceStruct;
}

crate trait GetUserExperience {
    fn get_user_experience(self, guild_id: GuildId, user_id: UserId) -> GetUserExperienceStruct;
}

crate trait GetGuildLeaderboard {
    fn get_guild_leaderboard(self, guild_id: GuildId) -> GetGuildLeaderboardStruct;
}

impl AddUserExperience for Client {
    fn add_user_experience(self, guild_id: GuildId, user_id: UserId, experience: u64) -> AddUserExperienceStruct {
        AddUserExperienceStruct::new(guild_id, user_id, experience)
    }
}

impl GetUserExperience for Client {
    fn get_user_experience(self, guild_id: GuildId, user_id: UserId) -> GetUserExperienceStruct {
        GetUserExperienceStruct::new(guild_id, user_id)
    }
}

impl GetGuildLeaderboard for Client {
    fn get_guild_leaderboard(self, guild_id: GuildId) -> GetGuildLeaderboardStruct {
        GetGuildLeaderboardStruct::new(guild_id)
    }
}
