/*
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # The `handler` Module
//!
//! This module defines the `EventHandler` struct, which defines various function handlers for
//! individual events.

use hartex_core::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        gateway::CloneableCluster,
        http::CloneableClient,
        model::gateway::{
            event::shard::Identifying,
            payload::{
                incoming::{
                    GuildCreate,
                    InteractionCreate,
                    MessageCreate,
                    Ready
                },
                outgoing::update_presence::UpdatePresence
            },
            presence::{
                Activity,
                ActivityType,
                Status
            }
        }
    },
    error::{
        HarTexError,
        HarTexResult
    },
    logging::tracing::{
        self,
        Instrument
    }
};
use hartex_dbmani::{
    guildconf::GetGuildConfig,
    whitelist::GetWhitelistedGuilds
};
use hartex_eventsys::emitter::EventEmitter;
use hartex_model::payload::CommandExecuted;
use hartex_plugins::{
    globadmin_only::refroles::Refroles,
    global::{
        about::About,
        ping::Ping,
        source::Source,
        team::Team
    },
    information::{
        guildinfo::Guildinfo,
        userinfo::Userinfo
    }
};
use tokio::time;

use crate::commands;

/// # Struct `EventHandler`
///
/// This structure defines various function handlers for individual events.
#[allow(clippy::module_name_repetitions)]
pub struct EventHandler;

// Twilight Events
impl EventHandler {
    /// # Static Asynchronous Method `EventHandler::guild_create`
    ///
    /// Handles the `GuildCreate` event.
    ///
    /// ## Parameters
    /// - `payload`, type `Box<GuildCreate>`: the `GuildCreate` event payload
    /// - `http`, type `Client`: the Twilight HTTP Client to use for sending a message to the guild
    ///                          owner about his/her guild's whitelist status if the guild is not
    ///                          in the whitelist or that the whitelist has been removed, or that
    ///                          the guild has been previously been whitelisted but the whitelist
    ///                          is deactivated with a reason.
    #[allow(clippy::missing_errors_doc)]
    pub async fn guild_create(
        payload: Box<GuildCreate>,
        http: CloneableClient
    ) -> HarTexResult<()> {
        let guild_id = payload.id;

        let span = tracing::trace_span!("event handler: guild create");
        span.in_scope(|| {
            tracing::trace!("joined a new guild with name `{name}` with id {guild_id}; checking whether the guild is whitelisted", name = payload.name);
        });

        let res = GetWhitelistedGuilds::default().await?;

        if !res.iter().any(|guild| guild_id == guild.GuildId) {
            span.in_scope(|| {
                tracing::error!("guild is not whitelisted");
            });

            let guild = http.guild(guild_id).exec().await?.model().await?;

            span.in_scope(|| {
                tracing::trace!("dming guild owner about the whitelist status");
            });

            let guild_owner = guild.owner_id;

            let user = http.user(guild_owner).exec().await?.model().await?;

            let dm_channel = http
                .create_private_channel(user.id)
                .exec()
                .await?
                .model()
                .await?;
            let message =
                "Hey there! It looks like you added HarTex to your guild by the name of \""
                    .to_string()
                    + &guild.name
                    + "\".\n\n"
                    + "Unfortunately, your guild has not been whitelisted yet and the bot cannot be "
                    + "invited to your guild until you apply for a whitelist and that the application is "
                    + "accepted.\n\n"
                    + "You may apply for a guild whitelist if your guild meets the following criteria, they include, but not limited to:\n"
                    + " - guild member count of at least 250;"
                    + " - be always abide by the Discord Terms of Service (<https://discord.com/terms>) and Community Guidelines (<https://discord.com/guidelines);"
                    + " - how old is the guild and/or how active is it; and"
                    + " - your experience level with TOML to configure the bot before using it.\n\n"
                    + "You may join our Support Guild at `https://discord.gg/Xu8453VBAv` for more information, including the application link in which you may use"
                    + "to apply for a whitelist application. Good luck!";

            http.create_message(dm_channel.id)
                .content(&message)?
                .exec()
                .await?;

            span.in_scope(|| {
                tracing::error!("leaving guild");
            });

            http.leave_guild(guild_id).exec().await?;

            return Err(HarTexError::Custom {
                message: String::from("guild is not whitelisted")
            });
        }

        span.in_scope(|| {
            tracing::info!("guild is whitelisted");
        });

        Ok(())
    }

    /// # Static Asynchronous Method `EventHandler::interaction_create`
    ///
    /// Handles the `InteractionCreate` event.
    ///
    /// ## Parameters
    /// - `payload`, type `Box<InteractionCreate>`: the `InteractionCreate` event payload
    /// - `http`, type `Client`: the Twilight HTTP client to pass to the command if the message is indeed a command
    /// - `cluster`, type `Cluster`: the Twilight gateway cluster
    /// - `cache`, type `InMemoryCache`: the Twilight in-memory cache
    /// - `emitter`, type `EventEmitter`: the event emitter
    #[allow(clippy::missing_errors_doc)]
    pub async fn interaction_create(
        payload: Box<InteractionCreate>,
        http: CloneableClient,
        cluster: CloneableCluster,
        cache: CloneableInMemoryCache,
        emitter: EventEmitter
    ) -> HarTexResult<()> {
        let span = tracing::trace_span!("event handler: interaction create");
        span.in_scope(|| {
            tracing::trace!("received an interaction, invoking interaction handler");
        });

        crate::interactions::handle_interaction(payload.0, cache, http, cluster, emitter).await?;

        Ok(())
    }

    /// # Static Asynchronous Method `EventHandler::message_create`
    ///
    /// Handles the `MessageCreate` event.
    ///
    /// ## Parameters
    /// - `payload`, type `Box<MessageCreate>`: the `MessageCreate` event payload
    /// - `emitter`, type `EventEmitter`: the event emitter to use when the message contains an actual command to execute
    /// - `cache`, type `InMemoryCache`: the cache to pass to the command if the message is indeed a command
    /// - `http`, type `Client`: the Twilight HTTP client to pass to the command if the message is indeed a command
    /// - `cluster`, type `Cluster`: the Twilight gateway cluster
    #[allow(clippy::missing_errors_doc)]
    pub async fn message_create(
        _: Box<MessageCreate>,
        _: EventEmitter,
        _: CloneableInMemoryCache,
        _: CloneableClient,
        _: CloneableCluster
    ) -> HarTexResult<()> {
        Ok(())
    }

    /// # Static Asynchronous Method `EventHandler::ready`
    ///
    /// Handles the `Ready` event.
    ///
    /// ## Parameters
    /// - `payload`, type `Box<Ready>`: the `Ready` event payload
    /// - `cluster`, type `Cluster`: the gateway cluster
    /// - `http`, type `Client`: the http client
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::too_many_lines)]
    pub async fn ready(
        payload: Box<Ready>,
        cluster: CloneableCluster,
        http: CloneableClient
    ) -> HarTexResult<()> {
        let span = tracing::info_span!("event handler: ready");
        span.in_scope(|| {
            let user = payload.user;

            tracing::info!(
                "{}#{} [id: {}] has successfully startup; using discord api v{}",
                user.name,
                user.discriminator,
                user.id,
                payload.version
            );
        });

        let span = tracing::trace_span!("event handler: ready: shard presences");

        async {
            for shard in cluster.shards() {
                let shard_id = match shard.info() {
                    Ok(info) => info,
                    Err(error) => {
                        tracing::error!("the shard session is inactive: {error}");
                        break;
                    }
                }
                .id();

                tracing::trace!("attempting to register presence for shard {shard_id}");

                match shard
                    .command(
                        &UpdatePresence::new(
                            vec![
                                Activity {
                                    application_id: None,
                                    assets: None,
                                    buttons: Vec::new(),
                                    created_at: None,
                                    details: None,
                                    emoji: None,
                                    flags: None,
                                    id: None,
                                    instance: None,
                                    kind: ActivityType::Watching,
                                    name: format!("codebase revamp | shard {}", shard_id),
                                    party: None,
                                    secrets: None,
                                    state: None,
                                    timestamps: None,
                                    url: None
                                },
                            ],
                            false,
                            None,
                            Status::Online
                        )
                        .unwrap()
                    )
                    .await
                {
                    Ok(()) => {
                        tracing::trace!("successfully set presence for shard {shard_id}");
                    }
                    Err(error) => {
                        tracing::error!("failed to set presence for shard {shard_id}: {error}");
                    }
                }
            }
        }
        .instrument(span)
        .await;

        let span = tracing::trace_span!("event handler: ready: global command registration");

        commands::register_global_commands(
            vec![
                // Global Administrator Only Plugin
                Box::new(Refroles),
                // Global Plugin
                Box::new(About),
                Box::new(Ping),
                Box::new(Source),
                Box::new(Team),
                // Information Plugin
                Box::new(Guildinfo),
                Box::new(Userinfo),
            ],
            http.clone()
        )
        .instrument(span)
        .await?;

        let span = tracing::trace_span!("event handler: ready: change guild nickname");

        for guild in http.current_user_guilds().exec().await?.models().await? {
            span.in_scope(|| {
                tracing::trace!("changing nickname in guild {name}", name = guild.name);
            });

            let config = match GetGuildConfig::new(guild.id).await {
                Ok(config) => {
                    span.in_scope(|| {
                        tracing::trace!("successfully retrieved guild config");
                    });

                    config
                }
                Err(error) => {
                    span.in_scope(|| {
                        tracing::error!("failed to retrieve guild config: {error:?}");
                    });

                    return Err(error);
                }
            };

            if let Err(error) = http
                .update_current_member(guild.id)
                .nick(Some(&config.GuildConfiguration.nickname))
                .unwrap()
                .exec()
                .await
            {
                span.in_scope(|| {
                    tracing::error!("failed to change nickname: {error}");
                });
            }

            time::sleep(time::Duration::from_secs(1)).await;
        }

        Ok(())
    }

    /// # Static Asynchronous Method `EventHandler::shard_identifying`
    ///
    /// Handles the `Identifying` event.
    ///
    /// ## Parameters
    ///
    /// - `payload`, type `Identifying`: the `Identifying` event payload
    #[allow(clippy::missing_errors_doc)]
    pub async fn shard_identifying(payload: Identifying) -> HarTexResult<()> {
        let span = tracing::trace_span!("event handler: shard identifying");
        span.in_scope(|| {
            tracing::trace!(
                "shard {} is identifying with the discord gateway",
                payload.shard_id
            );
        });

        Ok(())
    }
}

// Custom Events
// TODO: actually make use of these events
impl EventHandler {
    /// # Static Asynchronous Method `EventHandler::command_executed`
    ///
    /// Handles the `CommandExecuted` event.
    ///
    /// ## Parameters
    ///
    /// - `payload`, type `Box<CommandExecuted>`: the `CommandExecuted` event payload
    #[allow(clippy::missing_errors_doc)]
    pub async fn command_executed(payload: Box<CommandExecuted>) -> HarTexResult<()> {
        let span = tracing::trace_span!("event handler: command executed");
        span.in_scope(|| {
            tracing::trace!(
                "interaction command received (name: `{}`); invoking its handler",
                payload.command
            );
        });

        Ok(())
    }
}
