//! # The `config` Module
//!
//! This module contains configuration utilities for a command and the command parser.

/// # Struct `CommandConfig`
///
/// Holds the configuration for a command.
#[derive(Default)]
pub struct CommandConfig {
    name: String,
    aliases: Option<Vec<String>>
}

impl CommandConfig {
    /// # Static Method `CommandConfig::with_name`
    ///
    /// Creates a new `CommandConfig` with the given name.
    ///
    /// ## Parameters
    /// - `name`, type `String`: the command name
    pub fn with_name(name: String) -> Self {
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
    /// - `alias`, type `String`: the alias to add
    pub fn alias(mut self, alias: String) -> Self {
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
pub struct CommandParserConfig {
    commands: Vec<CommandConfig>
}

impl CommandParserConfig {
    /// # Instance Method `CommandParserConfig::command`
    ///
    /// Adds a command (its configuration) to the parser configuration.
    ///
    /// ## Parameters
    /// - `config`, type `CommandConfig`: the command configuratino to add to the parser
    ///                                   configuration
    pub fn command(mut self, config: CommandConfig) -> Self {
        self.commands.push(config);
        self
    }
}
