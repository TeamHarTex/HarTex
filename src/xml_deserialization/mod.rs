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

mod bot_config;
mod bot_customization;
mod dashboard;
crate mod plugin_management;
mod role_permission_level;
mod role_permission_levels;
mod user;

crate use bot_config::BotConfig;
crate use bot_customization::BotCustomization;
crate use dashboard::Dashboard;
crate use role_permission_level::RolePermissionLevel;
crate use role_permission_levels::RolePermissionLevels;
crate use user::User;
