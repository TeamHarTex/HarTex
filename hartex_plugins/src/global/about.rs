//! # The `about` Module
//!
//! This module implements the `about` command.

use hartex_cmdsys::{
    command::Command,
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
        model::channel::message::AllowedMentions
    },
    error::HarTexResult
};

use hartex_dbmani::whitelist::GetWhitelistedGuilds;

use hartex_utils::FutureRetType;

/// # Struct `About`
///
/// The `about` command.
pub struct About;

impl Command for About {
    fn name(&self) -> String {
        String::from("about")
    }

    fn execute(ctx: CommandContext, _: CommandArgs, _: InMemoryCache) -> FutureRetType<()> {
        Box::pin(exec_about_cmd(ctx))
    }
}

/// # Asynchronous Function `exec_about_cmd`
///
/// Executes the `about` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn exec_about_cmd(ctx: CommandContext) -> HarTexResult<()> {
    let whitelists = GetWhitelistedGuilds::default().await?.len();

    let embed = EmbedBuilder::new()
        .author(EmbedAuthorBuilder::new()
            .name("HarTex")
            .icon_url(ImageSource::url("https://cdn.discordapp.com/attachments/795539269925601341/862616114239897610/275a4a2ecfb5380a45c393c81838c14b.png")?)
        )
        .description("HarTex is a Discord bot that is built and optimized for efficient Discord moderation and administration, maintained by the HarTex Development Team members.")
        .color(0x03BEFC)
        .field(EmbedFieldBuilder::new("Whitelisted Guilds", whitelists.to_string()).inline().build())
        .build()?;

    ctx.http
        .create_message(ctx.message.channel_id)
        .embed(embed)?
        .reply(ctx.message.id)
        .allowed_mentions(AllowedMentions::default())
        .await?;

    Ok(())
}
