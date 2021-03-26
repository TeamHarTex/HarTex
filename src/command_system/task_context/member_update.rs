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

use std::{
    ops::Deref,
    sync::Arc
};

use twilight_http::Client;

use twilight_model::{
    guild::Member
};

crate struct MemberUpdateTaskContext(crate Arc<MemberUpdateTaskContextRef>);

impl Deref for MemberUpdateTaskContext {
    type Target = MemberUpdateTaskContextRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

crate struct MemberUpdateTaskContextRef {
    crate http_client: Client,
    crate member: Member
}

impl MemberUpdateTaskContextRef {
    crate fn new(http_client: Client, member: Member) -> MemberUpdateTaskContextRef {
        Self {
            http_client,
            member
        }
    }
}
