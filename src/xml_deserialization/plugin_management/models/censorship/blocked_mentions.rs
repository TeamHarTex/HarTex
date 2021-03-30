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

use std::{
    fmt::{
        Debug,
        Display,
        Formatter,
        Result as FmtResult
    }
};

use serde::{
    de::{
        Deserialize,
        Error as SerdeError,
        Visitor
    },
    Deserializer
};

#[derive(Clone, Debug, Serialize, Deserialize)]
crate struct BlockedMentions {
    #[serde(rename = "BlockedMention", default)]
    crate blocked_mentions: Vec<BlockedMention>
}

#[derive(Clone, Debug, Serialize)]
crate enum BlockedMention {
    ChannelId(u64),
    RoleId(u64),
    UserId(u64)
}

impl Deserialize<'deserialize> for BlockedMention {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'deserialize> {
        todo!()
    }
}

struct BlockedMentionVisitor;

impl Visitor<'_> for BlockedMentionVisitor {
    type Value = BlockedMention;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("A string to parse into a `BlockedMention`")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: SerdeError {
        if !v.ends_with(")") {
            return Err(SerdeError::custom(
                "method call does not end with `)`"
            ))
        }

        match &v[0..12] {
            "RoleIdOfU64" => {
                if let Ok(parsed) = &v[12..v.len() - 1].parse() {
                    Ok(BlockedMention::RoleId(*parsed))
                } else {
                    Err(SerdeError::custom("could not parse role ID passed into `RoleIdOfU64`"))
                }
            },
            "UserIdOfU64" => {
                if let Ok(parsed) = &v[12..v.len() - 1].parse() {
                    Ok(BlockedMention::UserId(*parsed))
                } else {
                    Err(SerdeError::custom("could not parse role ID passed into `UserIdOfU64`"))
                }
            },
            "ChannelIdOf" => {
                if let Ok(parsed) = &v[15..v.len() - 1].parse() {
                    Ok(BlockedMention::UserId(*parsed))
                } else {
                    Err(SerdeError::custom("could not parse role ID passed into `ChannelIdOfU64`"))
                }
            }
            _ => Err(
                SerdeError::custom("method call not found")
            )
        }
    }
}
