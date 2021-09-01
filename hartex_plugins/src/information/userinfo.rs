//! # The `userinfo` Module
//!
//! This module implements the `userinfo` command.

use hartex_cmdsys::{
    command::{
        Command,
        SlashCommand
    },
    context::CommandContext,
    parser::args::CommandArgs
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
        mention::ParseMention,
        model::{
            application::{
                callback::{
                    CallbackData,
                    InteractionResponse
                },
                interaction::Interaction
            },
            channel::message::AllowedMentions,
            id::UserId
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

impl Command for Userinfo {
    fn name(&self) -> String {
        String::from("userinfo")
    }

    fn execute_command(&self, ctx: CommandContext, mut args: CommandArgs, _: InMemoryCache) -> FutureRetType<()> {
        let id = args.next().unwrap_or("");
        let user_id = UserId::parse(id.clone())
            .map_or(None, |id| Some(id));

        Box::pin(exec_userinfo_cmd(ctx, user_id))
    }
}

/// # Asynchronous Function `exec_userinfo_cmd`
///
/// Executes the `userinfo` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn exec_userinfo_cmd(ctx: CommandContext, user_id: Option<UserId>) -> HarTexResult<()> {
    let message = ctx.message.unwrap();
    let _ = if let Some(id) = user_id {
        let result = ctx.http.user(id).exec().await;

        if result.is_err() {
            ctx.http
                .create_message(message.channel_id)
                .allowed_mentions(AllowedMentions::default())
                .content(&format!("could not retrieve user {}`", id.0))?
                .reply(message.id)
                .exec()
                .await?;
        }
    }
    else {
        todo!()
    };

    todo!()
}