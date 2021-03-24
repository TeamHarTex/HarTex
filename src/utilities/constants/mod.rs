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

use twilight_model::{
    id::RoleId
};

crate const fn content_distribution_network_base_url() -> &'static str {
    "https://cdn.discordapp.com/"
}

crate const fn hartex_guild_owner() -> RoleId {
    RoleId(791588740270784512)
}

crate const fn verified_hartex_user() -> RoleId {
    RoleId(791588599661199410)
}

crate const fn hartex_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

crate const fn bot_support_server() -> &'static str {
    "https://discord.gg/s8qjxZK"
}
