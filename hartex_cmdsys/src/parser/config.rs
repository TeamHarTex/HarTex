//! # The `config` Module
//!
//! This module contains configuration utilities for a command and the command parser.

/// # Struct `CommandConfig`
///
/// Holds the configuration for a command.
#[derive(Clone, Debug, Default)]
pub struct CommandConfig<'a> {
    pub name: &'a str,
    pub aliases: Option<Vec<&'a str>>
}

impl<'a> CommandConfig<'a> {
    /// # Static Method `CommandConfig::with_name`
    ///
    /// Creates a new `CommandConfig` with the given name.
    ///
    /// ## Parameters
    /// - `name`, type `&'a str`: the command name
    pub fn with_name(name: &'a str) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    /// # Instance Method `CommandConfig::alias`
    ///
    /// Adds an alias to the current command.
    ///
    /// ## Parameters
    /// - `alias`, type `&'a str`: the alias to add
    pub fn alias(mut self, alias: &'a str) -> Self {
        if self.aliases.is_none() {
            self.aliases = Some(vec![alias]);

            return self;
        }

        self.aliases = self.aliases.map(|mut vec| {
            vec.push(alias);

            vec
        });

        self
    }
}

/// # Struct `CommandParserConfig`
///
/// The command parser configuration.
#[derive(Clone, Debug, Default)]
pub struct CommandParserConfig<'a> {
    pub commands: Vec<CommandConfig<'a>>
}

impl<'a> CommandParserConfig<'a> {
    /// # Instance Method `CommandParserConfig::command`
    ///
    /// Adds a command (its configuration) to the parser configuration.
    ///
    /// ## Parameters
    /// - `config`, type `CommandConfig`: the command configuratino to add to the parser
    ///                                   configuration
    pub fn command(mut self, config: CommandConfig<'a>) -> Self {
        self.commands.push(config);
        self
    }
}
