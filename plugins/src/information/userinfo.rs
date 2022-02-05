/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! The `userinfo` command.

use hartex_base::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        embed_builder::{EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource},
        model::{
            application::{
                callback::{CallbackData, InteractionResponse},
                command::{ChoiceCommandOptionData, CommandOption, CommandOptionType},
                interaction::{application_command::CommandOptionValue, Interaction},
            },
            gateway::presence::{ActivityType, Status},
            id::UserId,
        },
        util::{
            mention::{Mention, ParseMention},
            snowflake::Snowflake,
        },
    },
    error::{HarTexError, HarTexResult},
    is_stable,
    logging::tracing,
    time::{FixedOffset, TimeZone},
};
use hartex_cmdsys::{
    command::{Command, CommandType},
    context::CommandContext,
};
use hartex_conftoml::guildconf::tz::Timezone;
use hartex_dbmani::guildconf::GetGuildConfig;
use hartex_utils::{
    cdn::{Cdn, CdnResourceFormat},
    FutureRetType,
};

/// The `userinfo` command.
pub struct Userinfo;

impl Command for Userinfo {
    fn name(&self) -> String {
        String::from("userinfo")
    }

    fn description(&self) -> String {
        String::from("InformationPlugin.UserinfoCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        cache: CloneableInMemoryCache,
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_userinfo_command(ctx, cache))
    }

    fn optional_cmdopts(&self) -> Vec<CommandOption> {
        vec![CommandOption::String(ChoiceCommandOptionData {
            autocomplete: false,
            choices: vec![],
            description: String::from("(optional) the user to query the information"),
            name: String::from("user"),
            required: false,
        })]
    }
}

/// Executes the `userinfo` command.
#[allow(clippy::too_many_lines)]
async fn execute_userinfo_command(
    ctx: CommandContext,
    cache: CloneableInMemoryCache,
) -> HarTexResult<()> {
    let interaction = if let Interaction::ApplicationCommand(command) = ctx.interaction.clone() {
        command
    } else {
        tracing::error!("invalid interaction type: expected ApplicationCommand");

        return Err(HarTexError::Custom {
            message: String::from("invalid interaction type: expected ApplicationCommand"),
        });
    };

    if interaction.guild_id.is_none() || interaction.user.is_some() {
        ctx.http
            .interaction_callback(
                interaction.id,
                &interaction.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(String::from(
                        ":x: This command can only be used in a guild.",
                    )),
                    embeds: None,
                    flags: None,
                    tts: None,
                }),
            )
            .exec()
            .await?;
    }

    let config = GetGuildConfig::new(interaction.guild_id.unwrap()).await?;

    let options = interaction.data.options;
    let user = if options.is_empty() {
        // unwrapping here is fine as it is now ensured that the interaction is sent from a guild,
        // not in a user DM (which is the case when interaction.member is None)
        interaction.member.unwrap().user.unwrap()
    } else {
        // unwrapping here is fine because the command only accepts a "user" parameter and is
        // asserted to be of type "String"; therefore, the parameter must exist if the options vec
        // is not empty and must be with the name "user" and of type "String"
        let user_option = options
            .into_iter()
            .find(|option| {
                option.name == "user" && option.value.kind() == CommandOptionType::String
            })
            .unwrap();
        let user_id = if let CommandOptionValue::String(string) = user_option.value {
            UserId::parse(&string)
                .ok()
                .unwrap_or_else(|| UserId::new(string.parse().unwrap()).unwrap())
        } else {
            return Err(HarTexError::Custom {
                message: String::from("invalid command option type: expected Mentionable"),
            });
        };

        ctx.http.user(user_id).exec().await?.model().await?
    };
    let future = ctx
        .http
        // it is ok to unwrap here because it is already checked that the interaction is sent from
        // a guild (which its id should never be None)
        .guild_member(interaction.guild_id.unwrap(), user.id)
        .exec()
        .await;

    let avatar_url = if let Some(ref hash) = user.avatar {
        let format = if hash.starts_with("a_") {
            CdnResourceFormat::GIF
        } else {
            CdnResourceFormat::PNG
        };

        Cdn::user_avatar(user.id, hash, &format)
    } else {
        Cdn::default_user_avatar(user.discriminator)
    };

    let mut embed = EmbedBuilder::new()
        .author(
            EmbedAuthorBuilder::new(format!(
                "Information about {username}",
                username = &user.name
            ))
            .icon_url(ImageSource::url(avatar_url)?),
        )
        .color(0x0003_BEFC)
        .field(EmbedFieldBuilder::new("Username", user.name).inline())
        .field(EmbedFieldBuilder::new("Discriminator", user.discriminator.to_string()).inline())
        .field(EmbedFieldBuilder::new(
            "User ID",
            format!("{id}", id = user.id),
        ));

    if future.is_ok() {
        let member = future.unwrap().model().await?;

        let presence = cache.presence(interaction.guild_id.unwrap(), member.user.id);
        let mut roles = member
            .roles
            .iter()
            .filter_map(|role_id| cache.role(*role_id))
            .collect::<Vec<_>>();
        roles.sort_by(|prev_role, curr_role| curr_role.position.cmp(&prev_role.position));

        let temp = embed.clone();
        embed = temp
            .field(
                EmbedFieldBuilder::new(
                    "Guild Nickname",
                    member.nick.unwrap_or_else(|| String::from("none")),
                )
                .inline(),
            )
            .field(
                EmbedFieldBuilder::new(
                    "Highest Role in Guild",
                    if roles.is_empty() {
                        String::from("none")
                    } else {
                        roles.first().unwrap().mention().to_string()
                    },
                )
                .inline(),
            );

        if let Some(presence) = presence {
            let activities = presence.activities();

            embed = embed.field(EmbedFieldBuilder::new(
                "Status",
                match presence.status() {
                    Status::DoNotDisturb => "do not disturb",
                    Status::Idle => "idle",
                    Status::Invisible => "invisible",
                    Status::Offline => "offline",
                    Status::Online => "online",
                },
            ));

            if activities.is_empty() {
                embed = embed.field(EmbedFieldBuilder::new("Activities", "none"));
            } else {
                for activity in activities {
                    let temp = embed.clone();
                    let activity_type = match activity.kind {
                        ActivityType::Competing => "Competing",
                        ActivityType::Custom => "Custom",
                        ActivityType::Listening => "Listening",
                        ActivityType::Playing => "Playing",
                        ActivityType::Streaming => "Streaming",
                        ActivityType::Watching => "Watching",
                    };

                    embed = temp.field(EmbedFieldBuilder::new(
                        format!("Activity - {activity_type}"),
                        if activity.kind == ActivityType::Custom {
                            activity.state.as_ref().unwrap()
                        } else {
                            &activity.name
                        },
                    ));
                }
            }
        } else {
            embed = embed
                .field(EmbedFieldBuilder::new("Status", "unknown"))
                .field(EmbedFieldBuilder::new("Activities", "none"));
        }

        let timezone = if config.NightlyFeatures.localization && !is_stable() {
            config.GuildConfiguration.timezone
        } else {
            Timezone::UTC
        };
        let joined_at = member.joined_at.iso_8601();
        let created_at =
            FixedOffset::east(timezone.into_offset_secs()).timestamp_millis(user.id.timestamp());

        let temp = embed.clone();

        embed = temp
            .field(
                EmbedFieldBuilder::new("Joined Guild At", format!("{joined_at} ({timezone})"))
                    .inline(),
            )
            .field(
                EmbedFieldBuilder::new("Account Created At", format!("{created_at} ({timezone})"))
                    .inline(),
            );
    }

    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(CallbackData {
                allowed_mentions: None,
                components: None,
                content: None,
                embeds: Some(vec![embed.build()?]),
                flags: None,
                tts: None,
            }),
        )
        .exec()
        .await?;

    Ok(())
}
