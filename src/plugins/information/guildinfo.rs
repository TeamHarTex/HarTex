//!  Copyright 2020 - 2021 The HarTex Project Developers
//!
//!  Licensed under the Apache License, Version 2.0 (the "License");
//!  you may not use this file except in compliance with the License.
//!  You may obtain a copy of the License at
//!
//!      http://www.apache.org/licenses/LICENSE-2.0
//!
//!  Unless required by applicable law or agreed to in writing, software
//!  distributed under the License is distributed on an "AS IS" BASIS,
//!  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!  See the License for the specific language governing permissions and
//!  limitations under the License.

use std::{
    future::Future,
    pin::Pin
};

use chrono::{
    Local,
    TimeZone
};

use twilight_cache_inmemory::InMemoryCache;

use twilight_embed_builder::{
    EmbedBuilder,
    EmbedFieldBuilder,
    ImageSource
};

use twilight_model::{
    channel::{
        message::AllowedMentions,
        GuildChannel,
    }, 
    guild::{
        PremiumTier,
        VerificationLevel
    },
};

use twilight_util::{
    snowflake::Snowflake
};

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
};

use crate::system::{
    SystemResult
};

crate struct GuildinfoCommand;

impl Command for GuildinfoCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("guild-info")
    }

    fn aliases(&self) -> Vec<String> {
        vec![String::from("server-info")]
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>, _arguments: Arguments,
                                            cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(information_guildinfo_command(ctx, cache))
    }
}

async fn information_guildinfo_command(ctx: CommandContext<'_>, cache: InMemoryCache)
    -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;
    let guild = cache.guild(ctx.message.guild_id.unwrap()).unwrap();
    let http_guild = ctx.http_client.guild(ctx.message.guild_id.unwrap())
        .await?;
    let guild_members = cache.guild_members(guild.id).unwrap();
    let guild_channels = cache.guild_channels(guild.id).unwrap();
    let guild_emojis = ctx.http_client.clone().emojis(guild.id).await?;

    let guild_roles_len = match http_guild.clone() {
        Some(ref guild) => {
            guild.roles.len()
        },
        None => 0
    };

    let owner = cache.user(guild.owner_id).unwrap();
    let mut features = String::new();

    match http_guild.clone() {
        Some(ref guild) => {
            guild.features.iter().for_each(|string| {
                features.push_str(format!("- {}\n", string).as_str())
            })
        },
        None => features.push_str("none")
    }

    let mut categories: u32 = 0;
    let mut texts: u32 = 0;
    let mut voices: u32 = 0;
    let mut stages: u32 = 0;

    for ch_id in guild_channels {
        if let Some(channel) =  cache.guild_channel(ch_id) {
            match *channel {
                GuildChannel::Category(_) => {
                    categories += 1;
                },
                GuildChannel::Text(_) => {
                    texts += 1;
                },
                GuildChannel::Voice(_) => {
                    voices += 1;
                },
                GuildChannel::Stage(_) => {
                    stages += 1;
                }
            }
        }
    }

    let embed = EmbedBuilder::new()
        .title(format!("Information About Guild {}", guild.clone().name))
        .thumbnail(ImageSource::url(format!("https://cdn.discordapp.com/icons/{}/{}",
                                            guild.id, guild.icon.clone().unwrap()))?)
        .color(0x03_BE_FC)
        .field(EmbedFieldBuilder::new("Guild ID", format!("{}", guild.id)))
        .field(EmbedFieldBuilder::new("Owner",
                                      format!("{}#{}", owner.name, owner.discriminator)))
        .field(EmbedFieldBuilder::new("Members",
                                      format!("Humans: {}\nBots: {}\n**Grand Total:** {}",
                                              guild_members.iter().filter(|&&user_id|
                                                  !cache.user(user_id).unwrap().bot).count(),
                                              guild_members.iter().filter(|&&user_id|
                                                  cache.user(user_id).unwrap().bot).count(),
                                              guild_members.len())))
        .field(EmbedFieldBuilder::new("Emojis", format!("{}", guild_emojis.len())))
        .field(EmbedFieldBuilder::new("Maximum Members", format!("{}", guild.max_members.unwrap())))
        .field(EmbedFieldBuilder::new("Maximum Presences", if let Some(max) = guild.max_presences { max.to_string() } else { "unknown".to_string() }))
        .field(EmbedFieldBuilder::new("Channels",
                                      format!("Categories: {}\nText Channels: {}\nVoice Channels: {}\nStage Channels: {}", categories,
                                              texts, voices, stages)).inline())
        .field(EmbedFieldBuilder::new("Roles", format!("{}", guild_roles_len)).inline())
        .field(EmbedFieldBuilder::new("Guild Features", features))
        .field(EmbedFieldBuilder::new("Guild Voice Region", guild.region.clone()))
        .field(EmbedFieldBuilder::new("Guild Created At", format!("{}+08:00", 
            Local.timestamp_millis(guild.id.timestamp()).format("%Y-%m-%d %H:%M:%S"))))
        .field(EmbedFieldBuilder::new(
            format!("Premium - {}",
                    match guild.premium_tier {
                        PremiumTier::None => "Tier 0",
                        PremiumTier::Tier1 => "Tier 1",
                        PremiumTier::Tier2 => "Tier 2",
                        PremiumTier::Tier3 => "Tier 3"
                    }
            ),
            format!(
                "Premium Subscriptions: {}",
                match guild.premium_subscription_count {
                    Some(count) => count,
                    None => 0
                }
            )
        ))
        .field(EmbedFieldBuilder::new("Guild Verification Level", match guild.verification_level {
            VerificationLevel::None => "none",
            VerificationLevel::Low => "low",
            VerificationLevel::Medium => "medium",
            VerificationLevel::High => "high",
            VerificationLevel::VeryHigh => "very high"
        }))
        .build()?;

    ctx.http_client.create_message(channel_id).embed(embed)?.reply(ctx.message.id).allowed_mentions(AllowedMentions::default())
        .await?;

    Ok(())
}
