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

extern crate serde;
extern crate quick_xml;

use crate::{
    xml_deserialization::{
        plugin_management::{
            levels::censorship::CensorshipLevel
        }
    }
};

#[derive(Debug, Clone, Serialize, Deserialize)]
crate struct CensorshipPlugin {
    #[serde(rename = "CensorshipLevels", default)]
    crate censorship_levels: CensorshipLevels
}

#[derive(Debug, Clone, Serialize, Deserialize)]
crate struct CensorshipLevels {
    #[serde(rename = "CensorshipLevel", default)]
    crate levels: Vec<CensorshipLevel>
}
