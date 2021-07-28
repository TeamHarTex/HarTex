//! # The `ping` Module
//!
//! This module implements the `ping` command.

use std::time::Duration;

use hartex_cmdsys::{
    command::Command,
    context::CommandContext,
    parser::args::CommandArgs
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        gateway::shard::raw_message::Message as RawGatewayMessage,
        model::channel::message::AllowedMentions
    },
    error::HarTexResult
};

use hartex_utils::{
    stopwatch::Stopwatch,
    FutureRetType,
    shard_id
};

/// # Struct `Ping`
///
/// The `ping` command.
pub struct Ping;

impl Command for Ping {
    fn name(&self) -> String {
        String::from("ping")
    }

    fn execute(ctx: CommandContext, _: CommandArgs, _: InMemoryCache) -> FutureRetType<()> {
        Box::pin(exec_ping_cmd(ctx))
    }
}

/// # Asynchronous Function `exec_ping_cmd`
///
/// Executes the `ping` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
/// - `cache`, type InMemoryCache`: the in-memory cache to use.
async fn exec_ping_cmd(ctx: CommandContext) -> HarTexResult<()> {
    let channel_id = ctx.message.channel_id;
    let message = ctx.http
        .create_message(channel_id)
        .reply(ctx.message.id)
        .allowed_mentions(AllowedMentions::default())
        .content("Hello! Did you need anything? :eyes:")?
        .await?;

    let shards = ctx.cluster.info();
    let shard_id = shard_id(ctx.message.guild_id.unwrap().0, shards.len() as _);
    let shard_info = shards
        .get(&shard_id)
        .unwrap();
    let latency = shard_info.latency().average()
        .unwrap();
    let new_content = format!("{} - `{}ms`", message.content, latency.as_millis());

    ctx.http
        .update_message(channel_id, message.id)
        .content(new_content)?
        .allowed_mentions(AllowedMentions::default())
        .await?;

    Ok(())
}