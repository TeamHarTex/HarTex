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

use sha3::{
    Sha3_224,
    Digest
};

use crate::{
    command_system::{
        Task,
        TaskContext
    },
    system::{
        model::infractions::InfractionType,
        twilight_http_client_extensions::{
            AddUserInfraction,
            GetGuildConfiguration
        },
        twilight_id_extensions::IntoInnerU64,
        SystemResult,
    },
    utilities::{
        invite_detection::invite_detected
    },
    xml_deserialization::{
        plugin_management::{
            models::channel_id::ChannelId
        },
        BotConfig
    }
};

crate struct InviteDetectionTask;

impl Task for InviteDetectionTask {
    fn execute_task<'asynchronous_trait>(ctx: TaskContext, config: BotConfig)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(censorship_invite_detection_task(ctx, config))
    }
}

async fn censorship_invite_detection_task(ctx: TaskContext, config: BotConfig) -> SystemResult<()> {
    if let TaskContext::MessageCreate(payload) = ctx {
        let message = payload.message.clone();

        if let Some(invite) = invite_detected(message.content) {
            if let Some(ref plugin) = config.plugins {
                if let Some(ref censorship) = plugin.censorship_plugin {
                    for level in &censorship.censorship_levels.levels {
                        if level.filter_invite_links == Some(true) {
                            if let Some(whitelisted_channels) = level.clone().invites_channel_whitelist {
                                if whitelisted_channels.channel_ids.contains(&ChannelId { id: message.channel_id.into_inner_u64() }) {
                                    return Ok(());
                                }
                            }

                            if let Some(whitelisted) = level.clone().whitelisted_guild_invites {
                                let twilight_invite = payload.http_client.clone().invite(invite.clone().code).await?;
                                let guild_vanity = payload.http_client.clone().guild_vanity_url(payload.message.guild_id.unwrap()).await?;

                                if let Some(actual_invite) = twilight_invite {
                                    if let Some(invite_guild) = actual_invite.guild {
                                        if !whitelisted.whitelists.iter()
                                            .any(|whitelist|
                                                whitelist.id == Some(invite_guild.id.into_inner_u64())
                                            ) {
                                            payload.http_client.clone()
                                                .delete_message(payload.message.channel_id, payload.message.id)
                                                .await?;

                                            if level.warn_on_censored == Some(true) {
                                                let warning_id = format!(
                                                    "{:x}",
                                                    Sha3_224::digest(
                                                        format!(
                                                            "{}{}{}",
                                                            payload.message.guild_id.unwrap().0,
                                                            payload.author.id.0,
                                                            String::from("Auto Moderation: Blocked invite censored.")
                                                        ).as_bytes()
                                                    )
                                                );

                                                payload.http_client.clone()
                                                    .add_user_infraction(warning_id,
                                                                         payload.message.guild_id.unwrap(),
                                                                         payload.message.author.id,
                                                                         String::from("Auto Moderation: Blocked invite censored."),
                                                                         InfractionType::Warning).await?;
                                            }
                                        }
                                        else if !whitelisted.whitelists.iter()
                                            .any(|whitelist| {
                                                if let (Some(vanity), Some(option_vanity)) = (whitelist.clone().vanity, guild_vanity.as_ref()) {
                                                    vanity == *option_vanity
                                                }
                                                else {
                                                    false
                                                }
                                            }) {
                                            payload.http_client.clone()
                                                .delete_message(payload.message.channel_id, payload.message.id)
                                                .await?;

                                            if level.warn_on_censored == Some(true) {
                                                let warning_id = format!(
                                                    "{:x}",
                                                    Sha3_224::digest(
                                                        format!(
                                                            "{}{}{}",
                                                            payload.message.guild_id.unwrap().0,
                                                            payload.author.id.0,
                                                            String::from("Auto Moderation: Blocked invite censored.")
                                                        ).as_bytes()
                                                    )
                                                );

                                                payload.http_client.clone()
                                                    .add_user_infraction(warning_id,
                                                                         payload.message.guild_id.unwrap(),
                                                                         payload.message.author.id,
                                                                         String::from("Auto Moderation: Blocked invite censored."),
                                                                         InfractionType::Warning).await?;
                                            }
                                        }
                                        else if let Some(blacklist) = level.clone().blacklisted_invite_codes {
                                            if blacklist.invite_codes.iter().any(|blacklisted_invite_code|
                                                blacklisted_invite_code.to_lowercase() == invite.clone().code.to_lowercase()) {
                                                payload.http_client.clone()
                                                    .delete_message(payload.message.channel_id, payload.message.id)
                                                    .await?;

                                                if level.warn_on_censored == Some(true) {
                                                    let warning_id = format!(
                                                        "{:x}",
                                                        Sha3_224::digest(
                                                            format!(
                                                                "{}{}{}",
                                                                payload.message.guild_id.unwrap().0,
                                                                payload.author.id.0,
                                                                String::from("Auto Moderation: Blocked invite censored.")
                                                            ).as_bytes()
                                                        )
                                                    );

                                                    payload.http_client.clone()
                                                        .add_user_infraction(warning_id,
                                                                             payload.message.guild_id.unwrap(),
                                                                             payload.message.author.id,
                                                                             String::from("Auto Moderation: Blocked invite censored."),
                                                                             InfractionType::Warning).await?;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
    else {
        unreachable!()
    }
}
