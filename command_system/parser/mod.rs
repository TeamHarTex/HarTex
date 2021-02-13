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

mod arguments;
mod case_sensitive;
mod configuration;

crate use arguments::Arguments;
crate use configuration::CommandParserConfiguration;

#[derive(Clone, Debug)]
#[non_exhaustive]
crate struct Command<'a> {
    pub arguments: Arguments<'a>,
    pub name: &'a str,
    pub prefix: &'a str,
}

#[derive(Clone, Debug)]
crate struct CommandParser<'a> {
    config: CommandParserConfiguration<'a>,
}

impl<'a> CommandParser<'a> {
    crate fn new(config: impl Into<CommandParserConfiguration<'a>>) -> Self {
        Self {
            config: config.into(),
        }
    }

    #[allow(dead_code)]
    crate fn configuration(&self) -> &CommandParserConfiguration<'a> {
        &self.config
    }

    #[allow(dead_code)]
    crate fn configuration_mut(&mut self) -> &mut CommandParserConfiguration<'a> {
        &mut self.config
    }

    crate fn parse(&'a self, buf: &'a str) -> Option<Command<'a>> {
        let prefix = self.find_prefix(buf)?;

        self.parse_with_prefix(prefix, buf)
    }

    crate fn parse_with_prefix(&'a self, prefix: &'a str, buf: &'a str) -> Option<Command<'a>> {
        if !buf.starts_with(prefix) {
            return None;
        }

        let mut idx = prefix.len();
        let command_buf = buf.get(idx..)?;
        let command = self.find_command(command_buf)?;

        idx += command.len();

        idx += command_buf.len() - command_buf.trim_start().len();

        Some(Command {
            arguments: Arguments::new(buf.get(idx..)?),
            name: command,
            prefix,
        })
    }

    fn find_command(&'a self, buf: &'a str) -> Option<&'a str> {
        let buf = buf.split_whitespace().next()?;
        self.config.commands.iter().find_map(|command| {
            if command == buf {
                Some(command.as_ref())
            } else {
                None
            }
        })
    }

    fn find_prefix(&self, buf: &str) -> Option<&str> {
        self.config.command_prefixes.iter().find_map(|prefix| {
            if buf.starts_with(prefix.as_ref()) {
                Some(prefix.as_ref())
            } else {
                None
            }
        })
    }
}

impl<'a, T: Into<CommandParserConfiguration<'a>>> From<T> for CommandParser<'a> {
    fn from(config: T) -> Self {
        Self::new(config)
    }
}
