//! # The `userinfo` Module
//!
//! This module implements the `userinfo` command

use hartex_cmdsys::{
    command::SlashCommand,
    context::CommandContext
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        embed_builder::{
            EmbedAuthorBuilder,
            EmbedBuilder,
            EmbedFieldBuilder,
            ImageSource
        },
        model::application::{
            callback::{
                CallbackData,
                InteractionResponse
            },
            command::{
                BaseCommandOptionData,
                CommandOption
            },
            interaction::Interaction
        }

    },
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_utils::FutureRetType;

/// # Struct `Userinfo`
///
/// The `userinfo` command.
pub struct Userinfo;

impl SlashCommand for Userinfo {
    fn name(&self) -> String {
        String::from("userinfo")
    }

    fn description(&self) -> String {
        String::from("InformationPlugin.UserinfoCommand")
    }

    fn execute_slash_command<'asynchronous_trait>(&self, _: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        todo!()
    }

    fn optional_cmdopts(&self) -> Vec<CommandOption> {
        vec![
            CommandOption::User(BaseCommandOptionData {
                description: String::from("(optional) the user to query the information"),
                name: String::from("user"),
                required: false
            })
        ]
    }
}