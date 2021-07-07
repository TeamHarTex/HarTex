//! # The `parser` Module
//!
//! This module contains an implementation of a command parser.

pub mod args;
pub mod config;

/// # Struct `CommandParser`
///
/// The command parser.
#[derive(Clone)]
pub struct CommandParser<'a> {
    config: config::CommandParserConfig<'a>
}

impl<'a> CommandParser<'a> {
    /// # Constructor `CommandParser::new`
    ///
    /// Creates a new `CommandParser` with the given `CommandParserConfig`.
    ///
    /// ## Parameters
    /// - `config`, type `CommandParserConfig`: the configuraiton to use for this parser
    pub fn new(config: config::CommandParserConfig<'a>) -> Self {
        Self {
            config
        }
    }

    /// # Instance Method `CommandParser::parse_command`
    ///
    /// Parses the given string as a command if possible, with the provided prefix.
    ///
    /// ## Parameters
    /// - `prefix`, type `&str`: the prefix to parse with
    /// - `buffer`, type `&str`: the string to parse with
    pub fn parse_command(&'a self, prefix: &'a str, buffer: &'a str) -> Option<ParsedCommand<'a>> {
        if !buffer.starts_with(prefix) {
            return None;
        }

        let mut index = prefix.len();
        let command_buffer = buffer.get(index..)?;
        let command = self.find_command(command_buffer)?;

        index += command.len();
        index += command_buffer.len() - command_buffer.trim_start().len();

        Some(ParsedCommand {
            args: args::CommandArgs::new(buffer.get(index..)?),
            name: command
        })
    }

    /// # Private Instance Method `CommandParser::find_command`
    ///
    /// Finds a command from the given string.
    ///
    /// ## Parameters
    /// - `buffer`, type `&str`: the string to find a command
    fn find_command(&'a self, buffer: &'a str) -> Option<&'a str> {
        let buffer = buffer.split_whitespace().next()?;

        self.config.commands.iter().find_map(|config| {
            let command = config.name;

            if command == buffer {
                return Some(command)
            }

            if config.aliases.is_none() {
                return None;
            }

            // it is now safe to call `.unwrap` on the value since we have checked beforehand
            // whether the value is `None`; would have definitely not been `None` if the code
            // reaches this branch
            config.aliases.as_ref().unwrap().iter().find(|&&alias| {
                alias == command
            })
                .map(|value| *value)
        })
    }
}

/// # Struct `ParsedCommand`
///
/// The command that is parsed by the parser.
pub struct ParsedCommand<'a> {
    pub args: args::CommandArgs<'a>,
    pub name: &'a str
}
