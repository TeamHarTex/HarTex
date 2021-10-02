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
            EmbedFieldBuilder,
            ImageSource
        },
        model::{
            application::{
                callback::{
                    CallbackData,
                    InteractionResponse,
                },
                interaction::Interaction,
            },
            channel::ChannelType
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

    fn execute<'asynchronous_trait>(&self, ctx: CommandContext, cache: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_guildinfo_command(ctx, cache))
    }
}

/// # Asynchronous Function `execute_guildinfo_command`
///
/// Executes the `guildinfo` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_guildinfo_command(ctx: CommandContext, cache: InMemoryCache) -> HarTexResult<()> {
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
    let guild = cache
        .guild(interaction.guild_id.unwrap())
        .unwrap();
    let guild_owner = ctx.http.user(guild.owner_id)
        .exec()
        .await?
        .model()
        .await?;
    let guild_members = ctx.http
        .guild_members(guild.id)
        .exec()
        .await?
        .models()
        .await?;
    let guild_channels = ctx.http
        .guild_channels(guild.id)
        .exec()
        .await?
        .models()
        .await?;

    let guild_member_count = guild_members.len();

    // FIXME: https://github.com/HarTexBot/HarTex-rust-discord-bot/issues/32
    let guild_user_count = guild_members
        .iter()
        .filter(|member| !member.user.bot)
        .count();

    let channels_iter = guild_channels
        .iter();

    let categories = channels_iter.clone()
        .filter(|channel| channel.kind() == ChannelType::GuildCategory)
        .count();
    let texts = channels_iter.clone()
        .filter(|channel| channel.kind() == ChannelType::GuildText)
        .count();
    let voices = channels_iter.clone()
        .filter(|channel| channel.kind() == ChannelType::GuildVoice)
        .count();
    let stages = channels_iter.clone()
        .filter(|channel| channel.kind() == ChannelType::GuildStageVoice)
        .count();
    let news = channels_iter
        .filter(|channel| channel.kind() == ChannelType::GuildNews)
        .count();

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

    let mut author = EmbedAuthorBuilder::new()
        .name(format!("Information about {name}", name = &guild.name));

    if !icon_url.is_empty() {
        let temp = author.clone();

        author = temp
            .icon_url(ImageSource::url(icon_url)?);
    }

    let embed = EmbedBuilder::new()
        .author(author)
        .color(0x03BEFC)
        .field(EmbedFieldBuilder::new("Guild Name", guild.name).inline())
        .field(EmbedFieldBuilder::new("Guild ID", format!("{id}", id = guild.id)).inline())
        .field(
            EmbedFieldBuilder::new(
                "Guild Owner",
                format!("{name}#{discriminator}", name = guild_owner.name, discriminator = guild_owner.discriminator)
            )
        )
        .field(EmbedFieldBuilder::new("Guild Owner User ID", format!("{id}", id = guild_owner.id)))
        .field(
            EmbedFieldBuilder::new(
                format!("Guild Members - {guild_member_count}"),
                format!(
                    "Humans: {guild_user_count}\nBots: {bots}",
                    bots = guild_member_count as usize - guild_user_count
                )
            )
        )
        .field(
            EmbedFieldBuilder::new(
                format!("Guild Channels - {total}", total = guild_channels.len()),
                format!(
                    "Categories: {categories}\nText Channels: {texts}\nVoice Channels: {voices}\nStage Channels: {stages}\nNews Channels: {news}"
                )
            )
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