//! # The `config` Module
//!
//! This module contains configuration utilities for a command and the command parser.

/// # Struct `CommandConfig`
///
/// Holds the configuration for a command.
#[derive(Default)]
pub struct CommandConfig {
    name: Option<String>,
    aliases: Option<Vec<String>>
}

impl CommandConfig {
    pub fn with_name(name: String) -> Self {
        Self {
            name: Some(name),
            ..Default::default()
        }
    }

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
    pub fn command(mut self, config: CommandConfig) -> Self {
        self.commands.push(config);
        self
    }
}
