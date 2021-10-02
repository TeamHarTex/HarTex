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
            channel::ChannelType,
            guild::VerificationLevel
        },
        util::snowflake::Snowflake
    },
    error::{
        HarTexError,
        HarTexResult
    },
    time::{
        FixedOffset,
        TimeZone
    }
};

use hartex_conftoml::guildconf::tz::Timezone;

use hartex_dbmani::guildconf::GetGuildConfig;

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

    let config = GetGuildConfig::new(interaction.guild_id.unwrap())
        .await?;

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
    // it is ok to call unwrap here because we are sure that the limit never exceeds 1000
    let guild_members = ctx.http
        .guild_members(guild.id)
        .limit(1000)
        .unwrap()
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
    /* let guild_voice_regions = ctx.http
        .guild_voice_regions(guild.id)
        .exec()
        .await?
        .models()
        .await?; */

    let guild_member_count = guild_members.len();

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

    /* let voice_regions_repr_str = guild_voice_regions
        .iter()
        .map(|region| format!("`{region}`", region = &region.name))
        .collect::<Vec<_>>(); */

    let mut embed = EmbedBuilder::new()
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
        /* .field(EmbedFieldBuilder::new("Guild Voice Region(s)", voice_regions_repr_str.join(", "))) */;

    let timezone = if config.NightlyFeatures.localization {
        config.GuildConfiguration.timezone
    }
    else {
        Timezone::UTC
    };

    let created_at = FixedOffset::east(timezone.into_offset_secs()).timestamp_millis(guild.id.timestamp());

    let features_vec = guild.features;
    let features = if features_vec.is_empty() {
        String::from("none")
    }
    else {
        let features_vec = features_vec
            .iter()
            .map(|feature| format!("`{feature}`"))
            .collect::<Vec<_>>();

        features_vec.join("\n - ")
    };

    let verification_level = match guild.verification_level {
        VerificationLevel::None => "none",
        VerificationLevel::Low => "low",
        VerificationLevel::Medium => "medium",
        VerificationLevel::High => "high",
        VerificationLevel::VeryHigh => "very high"
    };

    let temp = embed.clone();

    embed = temp
        .field(EmbedFieldBuilder::new("Guild Created At", format!("{created_at} ({timezone})")).inline())
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
        )
        .field(EmbedFieldBuilder::new("Guild Features", format!("- {features}")))
        .field(EmbedFieldBuilder::new("Guild Verification Level", verification_level));

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