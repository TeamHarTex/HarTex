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

use std::fmt::{
    Display,
    Formatter,
    Result
};

#[derive(Debug, Copy, Clone)]
crate enum InfractionType {
    Ban,
    Unban,
    Kick,
    Mute,
    TemporaryMute,
    Unmute,
    Warning
}

impl Display for InfractionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            Self::Ban => write!(f, "ban"),
            Self::Kick => write!(f, "kick"),
            Self::Mute => write!(f, "mute"),
            Self::TemporaryMute => write!(f, "temp-mute"),
            Self::Unmute => write!(f, "unmute"),
            Self::Warning => write!(f, "warning"),
            Self::Unban => write!(f, "unban"),
        }
    }
}

#[derive(Debug, Clone)]
crate struct Infraction {
    pub infraction_id: String,
    pub reason: String,
    pub infraction_type: InfractionType
}

impl Infraction {
    crate fn new(infraction_id: String, reason: String, infraction_type: InfractionType) -> Self {
        Self {
            infraction_id,
            reason,
            infraction_type
        }
    }
}
