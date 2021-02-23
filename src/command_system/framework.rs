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

use std::borrow::Cow;

use crate::command_system::{
    cfg::*,
    events::{
        events::{
            SystemEvent,
            CommandEvents
        },
        listener::Listeners
    },
    parser::{
        CommandParser,
        CommandParserConfiguration,
    },
    Command
};

#[derive(Clone)]
crate struct CommandFramework<'a> {
    command_parser_config: CommandParserConfiguration<'a>,
    listeners: Listeners<SystemEvent>
}

impl<'a> CommandFramework<'a> {
    crate fn new() -> Self {
        Self {
            command_parser_config: CommandParserConfiguration::default(),
            listeners: Listeners::default()
        }
    }

    crate fn command<T>(mut self, command: T, case_sensitive: CaseSensitivity, fully_qualified_name: UseFullyQualifiedName, enable_aliases: EnabledAliases) -> Self
    where T: Command {
        self
            .command_parser_config
            .add_command(
                match fully_qualified_name {
                    UseFullyQualifiedName::True => command.fully_qualified_name(),
                    UseFullyQualifiedName::False => command.name(),
                },
                case_sensitive
            );

        if matches!(enable_aliases, EnabledAliases::True) {
            for alias in command.aliases() {
                self.command_parser_config.add_command(alias, case_sensitive);
            }
        }

        self
    }

    crate fn command_prefix(mut self, prefix: impl Into<Cow<'a, str>>) -> Self {
        self.command_parser_config.add_prefix(prefix);

        self
    }

    crate fn build_parser(self) -> CommandParser<'a> {
        CommandParser::new(self.command_parser_config)
    }

    crate fn listeners(self) -> Listeners<SystemEvent> {
        self.listeners
    }

    crate fn events(self) -> CommandEvents {
        let rx = self.listeners.add();

        CommandEvents::new(rx)
    }
}
