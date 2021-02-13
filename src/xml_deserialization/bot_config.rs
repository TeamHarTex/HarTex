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

extern crate serde;
extern crate quick_xml;

use super::{
    plugin_management::{
        Plugins
    },
    BotCustomization,
    Dashboard,
    RolePermissionLevels
};

#[derive(Debug, Serialize, Deserialize)]
crate struct BotConfig {
    #[serde(rename = "Dashboard")]
    crate dashboard: Dashboard,

    #[serde(rename = "BotCustomization", default)]
    crate bot_customization: BotCustomization,

    #[serde(rename = "RolePermissionLevels", default)]
    crate role_permission_levels: RolePermissionLevels<u64, u32>,

    #[serde(rename = "Plugins")]
    crate plugins: Plugins
}
