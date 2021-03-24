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

#![feature(arbitrary_self_types)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]
#![feature(const_fn)]
#![feature(crate_visibility_modifier)]
#![feature(custom_inner_attributes)]
#![feature(decl_macro)]
#![feature(exclusive_range_pattern)]
#![feature(in_band_lifetimes)]
// In spite of this feature being incomplete, I will leave it there as I need it in the current existing codebase.
#![feature(let_chains)]
#![feature(once_cell)]

#![allow(clippy::needless_lifetimes)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_wraps)]
#![allow(incomplete_features)]

// Minimum Supported Rust Version
#![clippy::msrv = "1.53.0"]

// Globally Allow Warnings for now, will be removed in the future
#![allow(warnings)]

extern crate ctrlc;
#[macro_use]
extern crate serde_derive;
extern crate sha3;

use std::{
    env::*,
    error::Error,
    lazy::SyncLazy,
    pin::Pin,
    sync::Arc
};

use dotenv::dotenv;

use futures_util::{
    future::Either,
    stream::StreamExt
};

use sha3::{
    Digest,
    Sha3_512
};

use twilight_cache_inmemory::{
    ResourceType,
    InMemoryCache
};

use twilight_gateway::{
    cluster::{
        Cluster, 
        ShardScheme
    },
    Event,
    EventTypeFlags,
    Intents
};

use twilight_http::{
    request::{
        channel::reaction::RequestReactionType
    },
    Client as TwilightHttpClient
};

use twilight_model::{
    channel::{
        Message
    },
    gateway::{
        payload::{
            update_status::UpdateStatusInfo
        },
        presence::{
            Status
        },
    },
    id::{
        EmojiId
    }
};

crate mod command_system;
crate mod content_distribution_network;
crate mod logging;
crate mod macros;
crate mod models;
crate mod parsers;
crate mod plugins;
crate mod state_enums;
crate mod std_extensions;
crate mod system;
crate mod utilities;
crate mod xml_deserialization;

use crate::command_system::{
    cfg::*,
    events::{
        emitter::CommandEventEmitter,
        events::SystemEvent
    },
    parser::{
        Command,
        CommandParser
    },
    precommand_checks::{
        BotOwnerOnly,
        GuildIsAlreadySetup,
        GuildTextChannelOnly,
        GuildOwnerOnly,
        HasRolePermissions,
        PrecommandCheck,
        SupportGuildOnly
    },
    Command as CommandTrait,
    CommandContext,
    CommandContextRef,
    CommandFramework,
    PrecommandCheckParameters,
    PrecommandCheckParametersBuilder
};

use crate::logging::{
    logger::Logger
};

use crate::macros::{
    execute_command,
};

use crate::plugins::{
    administrator::{
        clean::{
            CleanAllCommand,
            CleanBotsCommand,
            CleanUserCommand
        },
        invites::{
            InvitesCommand
        },
        nickname_manipulation::{
            NicknameChangeCommand,
            NicknameRemoveCommand
        },
        lockdown::{
            LockdownChannelCommand,
            LockdownGuildCommand,
            UnlockdownChannelCommand,
            UnlockdownGuildCommand
        },
        roles::{
            noroles_manipulation::{
                NorolesKickCommand,
                NorolesListCommand
            },
            RoleAddCommand,
            RoleGlobalAddCommand,
            RoleGlobalRemoveCommand,
            RoleinfoCommand,
            RoleRemoveCommand
        },
        slowmode::{
            SlowmodeDisableChannelCommand,
            SlowmodeDisableHereCommand,
            SlowmodeEnableChannelCommand,
            SlowmodeEnableHereCommand
        },
        voice_manipulation::{
            VoicemuteDisableCommand,
            VoicemuteEnableCommand
        },
        WebconfigListCommand
    },
    general::{
        AboutCommand,
        HelpCommand,
        PingCommand,
        SourceCommand,
        TeamCommand,
        UptimeCommand
    },
    guild_owneronly::{
        SetupCommand
    },
    information::{
        BotinfoCommand,
        GuildinfoCommand,
        UserinfoCommand
    },
    infractions::{
        dm::{
            DmBanCommand,
            DmCleanBanCommand,
            DmKickCommand,
            DmMbanCommand,
            DmMkickCommand,
            DmMmuteCommand,
            DmMunbanCommand,
            DmMunmuteCommand,
            DmMuteCommand,
            DmMwarnCommand,
            DmTempbanCommand,
            DmTempmuteCommand,
            DmUnbanCommand,
            DmUnmuteCommand,
            DmWarnCommand,
        },
        infraction_manipulation::{
            InfractionClearallCommand,
            InfractionReasonCommand,
            InfractionRemoveCommand,
            InfractionSearchCommand,
            InfractionsArchiveCommand
        },
        nodm::{
            NodmBanCommand,
            NodmCleanBanCommand,
            NodmKickCommand,
            NodmMbanCommand,
            NodmMkickCommand,
            NodmMmuteCommand,
            NodmMunbanCommand,
            NodmMunmuteCommand,
            NodmMuteCommand,
            NodmMwarnCommand,
            NodmTempbanCommand,
            NodmTempmuteCommand,
            NodmUnbanCommand,
            NodmUnmuteCommand,
            NodmWarnCommand
        },
        SelfmuteCommand,
    },
    levelling_system::{
        RankCommand
    },
    owneronly::{
        RefreshWhitelistRolesCommand,
        RestartCommand,
        StopCommand,
        SupportAnnounceCommand,
        SupportinfoCommand
    },
    utilities::{
        AvatarCommand,
        CoinflipCommand,
        EmojiCommand,
        RandintCommand
    },
    whitelist::{
        AcceptCommand
    }
};

use crate::system::{
    bot_configuration::BotConfiguration,
    caching::SystemCache,
    event_handler::EventHandler,
    panicking::{
        RUST_DEFAULT_PANIC_HOOK
    },
    twilight_http_client_extensions::AddUserExperience,
    model::{
        payload::{
            CommandExecuted,
            CommandFailed,
            CommandReceived
        }
    },
    EventType,
    Stopwatch,
    SystemError,
    set_bot_activity,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Creates a new stopwatch.
    let stopwatch = Stopwatch::new();

    Logger::log_debug("Loading bot token from environment.");
    dotenv().ok();  // Loads the .env file.

    // Sets the token only if no errors are encountered when attempting to retrieve the token.
    let hartex_token = match var("HARTEX_TOKEN") {
        Ok(token) => {
            token
        },
        Err(_) => {
            Logger::log_error("Failed to retrieve bot token.");
            panic!("Failed to retrieve bot token.");
        }
    };

    Logger::log_debug("Bot token successfully retrieved.".to_string());

    // Creates a new configuration object.
    let bot_configuration = BotConfiguration::new(hartex_token);

    // Sharding scheme
    let shard_scheme = ShardScheme::Auto;

    Logger::log_debug("Building bot cluster.");
    Logger::log_debug("Registering gateway intents [ALL].");
    Logger::log_debug("Registering presence.");

    let intents =
        Intents::GUILDS |
        Intents::GUILD_MEMBERS |
        Intents::GUILD_BANS |
        Intents::GUILD_EMOJIS |
        Intents::GUILD_INTEGRATIONS |
        Intents::GUILD_WEBHOOKS |
        Intents::GUILD_INVITES |
        Intents::GUILD_VOICE_STATES |
        Intents::GUILD_PRESENCES |
        Intents::GUILD_MESSAGES |
        Intents::GUILD_MESSAGE_REACTIONS |
        Intents::GUILD_MESSAGE_TYPING |
        Intents::DIRECT_MESSAGES |
        Intents::DIRECT_MESSAGE_REACTIONS |
        Intents::DIRECT_MESSAGE_TYPING;

    let activities = vec![set_bot_activity()];

    // HarTex Bot cluster for Discord gateway
    let hartex_cluster = Cluster::builder(&bot_configuration.token, intents)
        .shard_scheme(shard_scheme)
        .presence(UpdateStatusInfo::new(activities, false, None, Status::Online))
        .build()
        .await?;

    Logger::log_debug("Bot cluster successfully created.");

    // Cloned cluster for tokio spawn
    let cluster_spawn = hartex_cluster.clone();

    Logger::log_debug("Initializing levelling cache.");
    let mut levelling_cache = SystemCache::new();

    // Spawns a tokio task to startup the cluster.
    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    Logger::log_debug("Building HTTP client.");

    // HarTex HTTP client
    let hartex_http = TwilightHttpClient::new(&bot_configuration.token);

    // HarTex command framework
    Logger::log_debug("Initializing command framework.");
    let framework = CommandFramework::new();

    // Initialization of command parser.
    let command_parser = {
        Logger::log_debug("Registering commands.");

        framework
            .clone()
            .command_prefix("hb.")

            // Administrator Command Module
            .command(CleanAllCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(RoleAddCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(RoleRemoveCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(RoleGlobalAddCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(RoleGlobalRemoveCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(RoleinfoCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(CleanUserCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(CleanBotsCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(LockdownChannelCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(UnlockdownChannelCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(SlowmodeEnableHereCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(SlowmodeDisableHereCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(SlowmodeEnableChannelCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(SlowmodeDisableChannelCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(VoicemuteEnableCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(VoicemuteDisableCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(NorolesListCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(NorolesKickCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(NicknameChangeCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(NicknameRemoveCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(WebconfigListCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(InvitesCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(LockdownGuildCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(UnlockdownGuildCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)

            // General Command Module
            .command(PingCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(HelpCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(AboutCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(TeamCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(UptimeCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(SourceCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            // Guild Owneronly Command Module
            .command(SetupCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            // Information Command Module
            .command(UserinfoCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(GuildinfoCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(BotinfoCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            // Infractions Command Module
            .command(InfractionSearchCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(InfractionRemoveCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(InfractionsArchiveCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(InfractionClearallCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)
            .command(InfractionReasonCommand, CaseSensitive, NoFullyQualifiedName, DisableAliases)

            .command(DmWarnCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmMuteCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmUnmuteCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmBanCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmKickCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmMkickCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmCleanBanCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmUnbanCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmTempmuteCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmMmuteCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmMwarnCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmMbanCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmTempbanCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmMunbanCommand, CaseSensitive, FullyQualifiedName, EnableAliases)
            .command(DmMunmuteCommand, CaseSensitive, FullyQualifiedName, EnableAliases)

            .command(NodmWarnCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmMuteCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmUnmuteCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmBanCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmKickCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmMkickCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmCleanBanCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmUnbanCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmTempmuteCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmMmuteCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmMwarnCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmMbanCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmTempbanCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmMunbanCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(NodmMunmuteCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            .command(SelfmuteCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            // Levelling System Command Module
            .command(RankCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            // Owneronly Command Module
            .command(RefreshWhitelistRolesCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(RestartCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(StopCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(SupportinfoCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(SupportAnnounceCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            // Utilities Command Module
            .command(CoinflipCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(EmojiCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(RandintCommand, CaseSensitive, FullyQualifiedName, DisableAliases)
            .command(AvatarCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            // Whitelist Command Module
            .command(AcceptCommand, CaseSensitive, FullyQualifiedName, DisableAliases)

            // Builds the parser with the configured commands and case sensitivity
            .build_parser()
    };

    let resource_types =
        ResourceType::CHANNEL |
        ResourceType::EMOJI |
        ResourceType::GUILD |
        ResourceType::MEMBER |
        ResourceType::MESSAGE |
        ResourceType::PRESENCE |
        ResourceType::REACTION |
        ResourceType::ROLE |
        ResourceType::USER |
        ResourceType::USER_CURRENT;

    let event_types =
        EventTypeFlags::BAN_ADD |
        EventTypeFlags::BAN_REMOVE |
        EventTypeFlags::CHANNEL_CREATE |
        EventTypeFlags::CHANNEL_DELETE |
        EventTypeFlags::CHANNEL_PINS_UPDATE |
        EventTypeFlags::CHANNEL_UPDATE |
        EventTypeFlags::GUILD_CREATE |
        EventTypeFlags::GUILD_DELETE |
        EventTypeFlags::GUILD_EMOJIS_UPDATE |
        EventTypeFlags::GUILD_INTEGRATIONS_UPDATE |
        EventTypeFlags::GUILD_UPDATE |
        EventTypeFlags::INVITE_CREATE |
        EventTypeFlags::INVITE_DELETE |
        EventTypeFlags::MEMBER_ADD |
        EventTypeFlags::MEMBER_CHUNK |
        EventTypeFlags::MEMBER_REMOVE |
        EventTypeFlags::MEMBER_UPDATE |
        EventTypeFlags::MESSAGE_CREATE |
        EventTypeFlags::MESSAGE_DELETE |
        EventTypeFlags::MESSAGE_DELETE_BULK |
        EventTypeFlags::MESSAGE_UPDATE |
        EventTypeFlags::READY |
        EventTypeFlags::SHARD_CONNECTED |
        EventTypeFlags::SHARD_CONNECTING |
        EventTypeFlags::SHARD_DISCONNECTED |
        EventTypeFlags::SHARD_IDENTIFYING |
        EventTypeFlags::SHARD_RECONNECTING |
        EventTypeFlags::USER_UPDATE |
        EventTypeFlags::VOICE_STATE_UPDATE |
        EventTypeFlags::WEBHOOKS_UPDATE;

    // Registering events
    let hartex_cache = InMemoryCache::builder()
        .resource_types(resource_types)
        .build();

    Logger::log_debug("Registered events.");

    // Framework Listeners
    let listeners = framework.clone().listeners();
    let emitter = CommandEventEmitter::new(listeners);

    // Cluster events
    let mut events = hartex_cluster.some_events(event_types);
    let mut command_events = framework.events();

    // Sets the Ctrl+C handler.
    ctrlc::set_handler(move || {
        Logger::log_warning("Received a Ctrl-C signal; terminating process.");

        std::process::exit(0);
    })?;

    SyncLazy::force(&RUST_DEFAULT_PANIC_HOOK);

    // Start an event loop to process each event in the stream as they come in.
    while let value = futures_util::future::select(
        StreamExt::next(&mut events), command_events.next()).await {
        let levelling_borrow = &mut levelling_cache;

        match value {
            Either::Left(event) => {
                hartex_cache.update(&event.0.clone().unwrap().1);

                let levelling = (*levelling_borrow).clone();

                tokio::spawn(
                    handle_event(
                        Some(event.0.clone().unwrap().0),
                        EventType::TwilightEvent,
                        Some(event.0.unwrap().1),
                        None,
                        hartex_http.clone(),
                        hartex_cluster.clone(),
                        command_parser.clone(),
                        hartex_cache.clone(),
                        stopwatch,
                        emitter.clone(),
                        levelling.clone()
                    )
                );
            },
            Either::Right(event) => {
                let levelling = (*levelling_borrow).clone();

                tokio::spawn(
                    handle_event(
                        None,
                        EventType::CustomEvent,
                        None,
                        event.0.clone(),
                        hartex_http.clone(),
                        hartex_cluster.clone(),
                        command_parser.clone(),
                        hartex_cache.clone(),
                        stopwatch,
                        emitter.clone(),
                        levelling
                    )
                );
            }
        }
    }

    Ok(())
}

async fn handle_event(_shard_id: Option<u64>,
                      event_type: EventType,
                      event: Option<Event>,
                      custom_event: Option<SystemEvent>,
                      http_client: TwilightHttpClient,
                      cluster: Cluster,
                      parser: CommandParser<'static>,
                      cache: InMemoryCache,
                      stopwatch: Stopwatch,
                      emitter: CommandEventEmitter,
                      levelling_cache: SystemCache<String, bool>)
    -> Result<(), Box<dyn Error + Send + Sync>> {
    match event_type {
        EventType::TwilightEvent => {
            if let Some(event) = event {
                match event {
                    Event::ShardConnecting(connecting) => {
                        EventHandler::shard_connecting(connecting).await
                    },
                    Event::ShardConnected(connected) => {
                        EventHandler::shard_connected(connected).await
                    },
                    Event::ShardIdentifying(identifying) => {
                        EventHandler::shard_identifying(identifying).await
                    },
                    Event::ShardReconnecting(reconnecting) => {
                        EventHandler::shard_reconnecting(reconnecting).await
                    },
                    Event::ShardDisconnected(disconnected) => {
                        EventHandler::shard_disconnected(disconnected).await
                    },
                    Event::Ready(ready) => {
                        EventHandler::ready(ready, stopwatch).await
                    },
                    Event::MessageCreate(message_create) => {
                        if (*message_create).author.bot {
                            return Ok(());
                        }

                        if message_create.content.starts_with("hb.") {
                            emitter.event(SystemEvent::CommandReceived(box CommandReceived));

                            match handle_command(
                                (*message_create).clone().0,
                                CommandContext(
                                    Arc::new(
                                        CommandContextRef::new(
                                            http_client.clone(),
                                            parser,
                                            cluster.clone(),
                                            (*message_create).clone().0,
                                            stopwatch
                                        )
                                    ),
                                ),
                                http_client.clone(),
                                cache,
                                emitter
                            ).await {
                                Ok(()) => (),
                                Err(error) => {
                                    Logger::log_error(format!("Failed to handle message: {}", error));

                                    let error_hash =
                                        format!("{:x}",
                                                Sha3_512::digest(format!("{}{:?}{:?}",
                                                                         error.to_string(),
                                                                         message_create.guild_id,
                                                                         message_create.id).as_str().as_bytes()));
                                    let message =
                                        format!(
                                            "Oops! This command raised an error. Please join or go to our Support Server (you need to get the **_HarTex** role, go to <#667597397215674368>) and provide the error code below for further troubleshooting and investigation.\n\nServer Invite: discord.gg/s8qjxZK\n\nError code: `{}`"
                                            , error_hash);

                                    http_client.clone().create_message(message_create.channel_id).content(message)?.await?;
                                }
                            }
                        }
                        else {
                            if message_create.content.to_lowercase().contains("harry") {
                                http_client
                                    .clone()
                                    .create_reaction(message_create.channel_id,
                                                     message_create.id, RequestReactionType::Custom {
                                            id: EmojiId(683744109550108704),
                                            name: None
                                        }).await?;
                            }

                            EventHandler::message_create(message_create.clone(), http_client.clone(), levelling_cache.clone()).await?;
                        }

                        Ok(())
                    },
                    Event::GuildCreate(guild_create) => {
                        EventHandler::guild_create(guild_create.clone(), http_client.clone()).await
                    },
                    _ => Ok(())
                }
            }
            else {
                return Err(box SystemError("EventType is TwilightEvent but event is None.".to_string()));
            }
        },
        EventType::CustomEvent => {
            if let Some(custom_event) = custom_event {
                match custom_event {
                    SystemEvent::CommandExecuted(command_executed) => {
                        EventHandler::command_executed(command_executed.clone()).await
                    },
                    SystemEvent::CommandFailed(command_failed) => {
                        EventHandler::command_failed(command_failed.clone()).await
                    }
                    SystemEvent::CommandReceived(command_received) => {
                        EventHandler::command_received(command_received.clone()).await
                    },
                    SystemEvent::CommandIdentified(command_identified) => {
                        EventHandler::command_identified(command_identified.clone()).await
                    }
                }
            }
            else {
                return Err(box SystemError("EventType is CustomEvent but custom_event is None.".to_string()));
            }
        }
    }
}

async fn handle_command(message: Message,
                        context: CommandContext<'static>,
                        http_client: TwilightHttpClient,
                        cache: InMemoryCache,
                        emitter: CommandEventEmitter) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(command) = context.command_parser.parse(&message.content) {
        emitter.event(SystemEvent::CommandIdentified(command.name.to_string()));

        match command {
            // Administrator Command Module
            Command { name: "clean", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("all") => {
                        execute_command!(
                            CleanAllCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "clean all"
                        );
                    },
                    Some("user") => {
                        execute_command!(
                            CleanUserCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "clean user"
                        );
                    },
                    Some("bots") => {
                        execute_command!(
                            CleanBotsCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "clean bots"
                        );
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "role", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("add") => {
                        execute_command!(
                            RoleAddCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "role add"
                        );
                    },
                    Some("remove") => {
                        execute_command!(
                            RoleRemoveCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "role remove"
                        );
                    },
                    Some("global-add") => {
                        execute_command!(
                            RoleGlobalAddCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(80).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "role global-add"
                        );
                    },
                    Some("global-remove") => {
                        execute_command!(
                            RoleGlobalRemoveCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(80).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "role global-remove"
                        );
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "role-info", arguments, .. } => {
                execute_command!(
                    RoleinfoCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params| Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(80).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "role-info"
                );
            },
            Command { name: "lockdown", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("channel") => {
                        execute_command!(
                            LockdownChannelCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                                |ctx, params|
                                    HasRolePermissions::execute_check(ctx, params)
                                , |ctx, params|
                                    GuildTextChannelOnly::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "lockdown channel"
                        );
                    },
                    Some("guild") => {
                        execute_command!(
                            LockdownGuildCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                                |ctx, params|
                                    HasRolePermissions::execute_check(ctx, params)
                                , |ctx, params|
                                    GuildTextChannelOnly::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "lockdown guild"
                        );
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "unlockdown", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("channel") => {
                        execute_command!(
                            UnlockdownChannelCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                                |ctx, params|
                                    HasRolePermissions::execute_check(ctx, params)
                                , |ctx, params|
                                    GuildTextChannelOnly::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "unlockdown channel"
                        );
                    },
                    Some("guild") => {
                        execute_command!(
                            UnlockdownGuildCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                                |ctx, params|
                                    HasRolePermissions::execute_check(ctx, params)
                                , |ctx, params|
                                    GuildTextChannelOnly::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "unlockdown guild"
                        );
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "slowmode", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("enable") => {
                        let position = arguments.next();

                        match position {
                            Some("here") => {
                                execute_command!(
                                    SlowmodeEnableHereCommand,
                                    Box::<[
                                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                                        |ctx, params|
                                            HasRolePermissions::execute_check(ctx, params)
                                        , |ctx, params|
                                            GuildTextChannelOnly::execute_check(ctx, params)
                                    ]),
                                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                                    context.clone(),
                                    arguments,
                                    cache.clone(),
                                    http_client.clone(),
                                    message,
                                    emitter.clone(),
                                    "slowmode enable here"
                                );
                            },
                            Some("channel") => {
                                execute_command!(
                                    SlowmodeEnableHereCommand,
                                    Box::<[
                                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                                        |ctx, params|
                                            HasRolePermissions::execute_check(ctx, params)
                                        , |ctx, params|
                                            GuildTextChannelOnly::execute_check(ctx, params)
                                    ]),
                                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                                    context.clone(),
                                    arguments,
                                    cache.clone(),
                                    http_client.clone(),
                                    message,
                                    emitter.clone(),
                                    "slowmode enable channel"
                                );
                            },
                            _ => Logger::log_error(
                                format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                        }
                    },
                    Some("disable") => {
                        let position = arguments.next();

                        match position {
                            Some("here") => {
                                execute_command!(
                                    SlowmodeDisableHereCommand,
                                    Box::<[
                                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                                        |ctx, params|
                                            HasRolePermissions::execute_check(ctx, params)
                                        , |ctx, params|
                                            GuildTextChannelOnly::execute_check(ctx, params)
                                    ]),
                                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                                    context.clone(),
                                    arguments,
                                    cache.clone(),
                                    http_client.clone(),
                                    message,
                                    emitter.clone(),
                                    "slowmode disable here"
                                );
                            },
                            Some("channel") => {
                                execute_command!(
                                    SlowmodeDisableChannelCommand,
                                    Box::<[
                                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                                        |ctx, params|
                                            HasRolePermissions::execute_check(ctx, params)
                                        , |ctx, params|
                                            GuildTextChannelOnly::execute_check(ctx, params)
                                    ]),
                                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                                    context.clone(),
                                    arguments,
                                    cache.clone(),
                                    http_client.clone(),
                                    message,
                                    emitter.clone(),
                                    "slowmode disable channel"
                                );
                            },
                            _ => Logger::log_error(
                                format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                        }
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "voicemute", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("enable") => {
                        execute_command!(
                            VoicemuteEnableCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| HasRolePermissions::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "voicemute enable"
                        );
                    },
                    Some("disable") => {
                        execute_command!(
                            VoicemuteDisableCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| HasRolePermissions::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "voicemute disable"
                        );
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "noroles", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("list") => {
                        execute_command!(
                            NorolesListCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| HasRolePermissions::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "noroles list"
                        );
                    },
                    Some("kick") => {
                        execute_command!(
                            NorolesKickCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| HasRolePermissions::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(80).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "noroles kick"
                        );
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "nickname", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("change") => {
                        execute_command!(
                            NorolesKickCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| HasRolePermissions::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "nickname change"
                        );
                    },
                    Some("remove") => {
                        execute_command!(
                            NicknameRemoveCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| HasRolePermissions::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "nickname remove"
                        );
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "webconfig", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("list") => {
                        execute_command!(
                            WebconfigListCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params| HasRolePermissions::execute_check(ctx, params)
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(80).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "webconfig list"
                        );
                    },
                    _ => Logger::log_error(
                        format!("Command '{}' failed due to an error: 'command not found'.", message.content))
                }
            },
            Command { name: "invites", arguments, .. } => {
                execute_command!(
                    WebconfigListCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params| HasRolePermissions::execute_check(ctx, params)
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(80).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "invites"
                );
            },

            // General Command Module
            Command { name: "ping", arguments, .. } => {
                execute_command!(
                    PingCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "ping"
                );
            },
            Command { name: "bot-info", arguments, .. } => {
                execute_command!(
                    BotinfoCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "bot-info"
                );
            },
            Command { name: "help", arguments, .. } => {
                execute_command!(
                    HelpCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "help"
                );
            },
            Command { name: "about", arguments, .. } => {
                execute_command!(
                    AboutCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "about"
                );
            },
            Command { name: "team", arguments, .. } | Command { name: "staff", arguments, .. } => {
                execute_command!(
                    TeamCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "team"
                );
            },
            Command { name: "uptime", arguments, .. } => {
                execute_command!(
                    UptimeCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "uptime"
                );
            },
            Command { name: "source", arguments, .. } => {
                execute_command!(
                    SourceCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "source"
                );
            },

            // Information Command Module
            Command { name: "user-info", arguments, .. } => {
                execute_command!(
                    UserinfoCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "user-info"
                );
            },
            Command { name: "guild-info", arguments, .. } | Command { name: "server-info", arguments, .. } => {
                execute_command!(
                    GuildinfoCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "guild-info"
                );
            },

            // Guild Owneronly Command Module
            Command { name: "setup", arguments, .. } => {
                execute_command!(
                    SetupCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                        |ctx, params|
                            Box::pin(GuildOwnerOnly::execute_check(ctx, params)),
                        |ctx, params|
                            Box::pin(GuildIsAlreadySetup::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).guild_id(context.clone().message.guild_id.unwrap()).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "setup"
                );
            },

            // Infractions Command Module
            Command { name: "warn", arguments, .. }  |
            Command { name: "dmwarn", arguments, .. } => {
                execute_command!(
                    DmWarnCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmwarn"
                );
            },
            Command { name: "nodmwarn", arguments, .. } => {
                execute_command!(
                    NodmWarnCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmwarn"
                );
            },
            Command { name: "inf", mut arguments, .. } => {
                let subcommand = arguments.next();

                match subcommand {
                    Some("search") => {
                        execute_command!(
                            InfractionSearchCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params|
                                    Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "inf search"
                        );
                    },
                    Some("remove") => {
                        execute_command!(
                            InfractionRemoveCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params|
                                    Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(80).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "inf remove"
                        );
                    },
                    Some("archive") => {
                        execute_command!(
                            InfractionsArchiveCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params|
                                    Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(80).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "inf archive"
                        );
                    },
                    Some("clear-all") => {
                        execute_command!(
                            InfractionClearallCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params|
                                    Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "inf clear-all"
                        );
                    },
                    Some("reason") => {
                        execute_command!(
                            InfractionReasonCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params|
                                    Box::pin(HasRolePermissions::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "inf reason"
                        );
                    },
                    _ => {
                        emitter.event(SystemEvent::CommandFailed(box CommandFailed {
                            command: "unknown",
                            error: String::from("command not found.")
                        }))
                    }
                }
            },
            Command { name: "mute", arguments, .. } |
            Command { name: "dmmute", arguments, .. } => {
                execute_command!(
                    DmMuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmmute"
                );
            },
            Command { name: "nodmmute", arguments, .. } => {
                execute_command!(
                    NodmMuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmmute"
                );
            },
            Command { name: "unmute", arguments, .. } |
            Command { name: "dmunmute", arguments, .. } => {
                execute_command!(
                    DmUnmuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmunmute"
                );
            },
            Command { name: "nodmunmute", arguments, .. } => {
                execute_command!(
                    NodmMuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmunmute"
                );
            },
            Command { name: "kick", arguments, .. } |
            Command { name: "dmkick", arguments, .. } => {
                execute_command!(
                    DmKickCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmkick"
                );
            },
            Command { name: "nodmkick", arguments, .. } => {
                execute_command!(
                    NodmMuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmkick"
                );
            },
            Command { name: "ban", arguments, .. } |
            Command { name: "dmban", arguments, .. } => {
                execute_command!(
                    DmBanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmban"
                );
            },
            Command { name: "nodmban", arguments, .. } => {
                execute_command!(
                    NodmBanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmban"
                );
            },
            Command { name: "mkick", arguments, .. } |
            Command { name: "dmmkick", arguments, .. } => {
                execute_command!(
                    DmMkickCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmmkick"
                );
            },
            Command { name: "nodmmkick", arguments, .. } => {
                execute_command!(
                    NodmMkickCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmmkick"
                );
            },
            Command { name: "cleanban", arguments, .. } |
            Command { name: "dmcleanban", arguments, .. } => {
                execute_command!(
                    DmCleanBanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmcleanban"
                );
            },
            Command { name: "nodmcleanban", arguments, .. } => {
                execute_command!(
                    NodmCleanBanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmcleanban"
                );
            },
            Command { name: "unban", arguments, .. } |
            Command { name: "dmunban", arguments, .. } => {
                execute_command!(
                    DmUnbanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmunban"
                );
            },
            Command { name: "nodmunban", arguments, .. } => {
                execute_command!(
                    NodmUnbanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmunban"
                );
            },
            Command { name: "tempmute", arguments, .. } |
            Command { name: "dmtempmute", arguments, .. } => {
                execute_command!(
                    DmTempmuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmtempmute"
                );
            },
            Command { name: "nodmtempmute", arguments, .. } => {
                execute_command!(
                    NodmTempmuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmtempmute"
                );
            },
            Command { name: "mmute", arguments, .. } |
            Command { name: "dmmmute", arguments, .. } => {
                execute_command!(
                    DmMmuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmmmute"
                );
            },
            Command { name: "nodmmmute", arguments, .. } => {
                execute_command!(
                    NodmMmuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmmmute"
                );
            },
            Command { name: "munmute", arguments, .. } |
            Command { name: "dmmunmute", arguments, .. } => {
                execute_command!(
                    DmMunmuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmmunmute"
                );
            },
            Command { name: "nodmmunmute", arguments, .. } => {
                execute_command!(
                    NodmMunmuteCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmmunmute"
                );
            },
            Command { name: "mwarn", arguments, .. } |
            Command { name: "dmmwarn", arguments, .. } => {
                execute_command!(
                    DmMwarnCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmmwarn"
                );
            },
            Command { name: "nodmmwarn", arguments, .. } => {
                execute_command!(
                    NodmMwarnCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmmwarn"
                );
            },
            Command { name: "mban", arguments, .. }|
            Command { name: "dmmban", arguments, .. } => {
                execute_command!(
                    DmMbanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmmban"
                );
            },
            Command { name: "nodmmban", arguments, .. } => {
                execute_command!(
                    NodmMbanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmmban"
                );
            },
            Command { name: "selfmute", arguments, .. } => {
                execute_command!(
                    SelfmuteCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "selfmute"
                );
            },
            Command { name: "munban", arguments, .. }|
            Command { name: "dmmunban", arguments, .. } => {
                execute_command!(
                    DmMunbanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "dmmunban"
                );
            },
            Command { name: "nodmmunban", arguments, .. } => {
                execute_command!(
                    NodmMunbanCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(HasRolePermissions::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().in_memory_cache(cache.clone()).minimum_permission_level(60).build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "nodmmunban"
                );
            },

            // Levelling System Command Module
            Command { name: "rank", arguments, .. } => {
                execute_command!(
                    RankCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "rank"
                );
            }

            // Owneronly Command Module
            Command { name: "restart", arguments, .. } => {
                execute_command!(
                    RestartCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(BotOwnerOnly::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "restart"
                );
            },
            Command { name: "stop", arguments, .. } => {
                execute_command!(
                    StopCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                        |ctx, params|
                            Box::pin(BotOwnerOnly::execute_check(ctx, params))
                    ]),
                    PrecommandCheckParametersBuilder::new().build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "stop"
                );
            },
            Command { name: "support-info", arguments, .. } => {
                execute_command!(
                    SupportinfoCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                        |ctx, params|
                            Box::pin(BotOwnerOnly::execute_check(ctx, params)),
                        |ctx, params|
                            SupportGuildOnly::execute_check(ctx, params)
                    ]),
                    PrecommandCheckParametersBuilder::new().build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "support-info"
                );
            },
            Command { name: "refresh-whitelist-roles", arguments, .. } => {
                execute_command!(
                    RefreshWhitelistRolesCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                        |ctx, params|
                            Box::pin(BotOwnerOnly::execute_check(ctx, params)),
                        |ctx, params|
                            SupportGuildOnly::execute_check(ctx, params)
                    ]),
                    PrecommandCheckParametersBuilder::new().build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "support-info"
                );
            },
            Command { name: "support-announce", arguments, .. } => {
                execute_command!(
                    SupportAnnounceCommand,
                    Box::<[
                        for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                    -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                        (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 2]>::new([
                        |ctx, params|
                            Box::pin(BotOwnerOnly::execute_check(ctx, params)),
                        |ctx, params|
                            SupportGuildOnly::execute_check(ctx, params)
                    ]),
                    PrecommandCheckParametersBuilder::new().build(),
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "support-announce"
                );
            },

            // Utilities Command Module
            Command { name: "emoji", arguments, .. } => {
                execute_command!(
                    EmojiCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "emoji"
                );
            },
            Command { name: "randint", arguments, .. } => {
                execute_command!(
                    RandintCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "randint"
                );
            },
            Command { name: "coinflip", arguments, .. } => {
                execute_command!(
                    CoinflipCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "coinflip"
                );
            },
            Command { name: "avatar", arguments, .. } => {
                execute_command!(
                    AvatarCommand,
                    context.clone(),
                    arguments,
                    cache.clone(),
                    http_client.clone(),
                    message,
                    emitter.clone(),
                    "avatar"
                );
            },

            // Whitelist Command Module
            Command { name: "whitelist", mut arguments, .. } => {
                let guild = match http_client.guild(message.guild_id.unwrap()).await? {
                    Some(guild) => guild.name,
                    None => String::new()
                };

                let subcommand = arguments.next();

                match subcommand {
                    Some("accept") => {
                        execute_command!(
                            AcceptCommand,
                            Box::<[
                                for<'asynchronous_trait> fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
                                                            -> Pin<Box<dyn std::future::Future<Output = std::result::Result<
                                                                (), Box<(dyn Error + Send + Sync)>>> + Send + 'asynchronous_trait>>; 1]>::new([
                                |ctx, params|
                                    Box::pin(BotOwnerOnly::execute_check(ctx, params))
                            ]),
                            PrecommandCheckParametersBuilder::new().build(),
                            context.clone(),
                            arguments,
                            cache.clone(),
                            http_client.clone(),
                            message,
                            emitter.clone(),
                            "whitelist accept"
                        );
                    },
                    _ => {
                        Logger::log_error(
                            format!(
                                "Command '{}' failed due to an error: 'command not found'.", message.content
                            )
                        );
                    }
                }
            },

            _ => ()
        }
    }
    else {
    }

    Ok(())
}
