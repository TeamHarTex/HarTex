//! # The `guildinfo` Module
//!
//! This module implements the `guildinfo` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType,
    },
    context::CommandContext
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        embed_builder::{
            EmbedAuthorBuilder,
            EmbedBuilder,
            ImageSource
        },
        model::application::{
            callback::{
                CallbackData,
                InteractionResponse
            },
            interaction::Interaction
        },
    },
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_utils::{
    cdn::{
        Cdn,
        CdnResourceFormat
    },
    FutureRetType
};

/// # Struct `Guildinfo`
///
/// The `guildinfo` command.
pub struct Guildinfo;

impl Command for Guildinfo {
    fn name(&self) -> String {
        String::from("guildinfo")
    }

    fn description(&self) -> String {
        String::from("InformationPlugin.GuildinfoCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(&self, ctx: CommandContext, _: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_guildinfo_command(ctx))
    }
}

/// # Asynchronous Function `execute_guildinfo_command`
///
/// Executes the `guildinfo` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_guildinfo_command(ctx: CommandContext) -> HarTexResult<()> {
    let interaction = match ctx.interaction.clone() {
        Interaction::ApplicationCommand(command) => command,
        _ => return Err(
            HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            }
        )
    };

    if interaction.guild_id.is_none() || interaction.user.is_some() {
        ctx.http
            .interaction_callback(
                interaction.id,
                &interaction.token,
                &InteractionResponse::ChannelMessageWithSource(
                    CallbackData {
                        allowed_mentions: None,
                        components: None,
                        content: Some(String::from(":x: This command can only be used in a guild.")),
                        embeds: vec![],
                        flags: None,
                        tts: None
                    }
                )
            )
            .exec()
            .await?;
    }

    // unwrapping here is fine as it is now ensured that the interaction is sent from a guild,
    // not in a user DM (which is the case when interaction.guild_id is None)
    let guild = ctx.http
        .guild(interaction.guild_id.unwrap())
        .exec()
        .await?
        .model()
        .await?;

    let icon_url = if let Some(hash) = guild.icon {
        let format = if hash.starts_with("a_") {
            CdnResourceFormat::GIF
        }
        else {
            CdnResourceFormat::PNG
        };

        Cdn::guild_icon(guild.id, hash, format)
    }
    else {
        String::new()
    };

    let mut embed = EmbedBuilder::new()
        .author(EmbedAuthorBuilder::new()
            .name(format!("Information about {name}", name = &guild.name))
            .icon_url(ImageSource::url(icon_url)?)
        );

    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(
                CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: None,
                    embeds: vec![embed.build()?],
                    flags: None,
                    tts: None
                }
            )
        )
        .exec()
        .await?;

    Ok(())
}