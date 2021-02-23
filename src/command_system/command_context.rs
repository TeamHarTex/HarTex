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

use twilight_gateway::{
    Cluster,
};

use twilight_http::{
    Client as HttpClient
};

use twilight_model::{
    channel::{
        Message
    },
    guild::{
        PartialMember
    },
    user::User,
};

use crate::{
    command_system::{
        parser::{
            CommandParser
        }
    },
    system::{
    Stopwatch
    }
};

#[derive(Clone)]
crate struct CommandContext<'a>(crate Arc<CommandContextRef<'a>>);

impl<'a> Deref for CommandContext<'a> {
    type Target = CommandContextRef<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
crate struct CommandContextRef<'a> {
    crate http_client: HttpClient,
    crate command_parser: CommandParser<'a>,
    crate cluster: Cluster,
    crate message: Message,
    crate author: User,
    crate member: Option<PartialMember>,
    crate stopwatch: Stopwatch
}

impl<'a> CommandContextRef<'a> {
    crate fn new(
        http_client: HttpClient, 
        command_parser: CommandParser<'a>,
        cluster: Cluster,
        message: Message,
        stopwatch: Stopwatch
    ) -> Self {
        let author = message.clone().author;
        let member = message.clone().member;

        CommandContextRef {
            http_client,
            command_parser,
            cluster,
            message,
            author,
            member,
            stopwatch
        }
    }
}
