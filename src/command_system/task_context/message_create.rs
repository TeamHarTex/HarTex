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
    channel::Message,
    user::User
};

crate struct MessageCreateTaskContext(crate Arc<MessageCreateTaskContextRef>);

impl Deref for MessageCreateTaskContext {
    type Target = MessageCreateTaskContextRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

crate struct MessageCreateTaskContextRef {
    crate http_client: Client,
    crate author: User,
    crate message: Message
}

impl MessageCreateTaskContextRef {
    crate fn new(http_client: Client, author: User, message: Message) -> Self {
        Self {
            http_client,
            author,
            message
        }
    }
}
