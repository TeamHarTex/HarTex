//! # The `parser` Module
//!
//! This module contains an implementation of a command parser.

pub mod args;
pub mod config;

/// # Struct `CommandParser`
///
/// The command parser.
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

    pub fn parse_command(&'a self, prefix: &'a str, buffer: &'a str) -> Option<ParsedCommand<'a>> {
        todo!()
    }
}

/// # Struct `ParsedCommand`
///
/// The command that is parsed by the parser.
pub struct ParsedCommand<'a> {
    pub args: args::CommandArgs<'a>,
    pub name: &'a str
}
