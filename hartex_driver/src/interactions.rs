use std::sync::Arc;

use hartex_cmdsys::{
    command::Command,
    context::{
        CommandContext,
        CommandContextInner
    }
};
use hartex_core::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        gateway::CloneableCluster,
        http::CloneableClient,
        model::application::interaction::Interaction
    },
    error::HarTexResult,
    logging::tracing::{
        self,
        Instrument
    }
};
use hartex_eventsys::{
    emitter::EventEmitter,
    events::HarTexEvent
};
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

/// # Asynchronous Function `handle_interaction`
///
/// Handles the incoming interaction asynchronously.
///
/// ## Parameters
/// - `interaction`, type `Interaction`: the interaction
/// - `cache`, type `InMemoryCache`: the in-memory cache
/// - `http`, type `Client`: the Twilight HTTP client
/// - `cluster`, type `Cluster: the gateway cluster
/// - `emitter`, type `EventEmitter`: the event emitter
///
/// ## Errors
///
/// Interaction-related errors.
#[allow(clippy::too_many_lines)]
pub async fn handle_interaction(
    interaction: Interaction,
    cache: CloneableInMemoryCache,
    http: CloneableClient,
    cluster: CloneableCluster,
    emitter: EventEmitter
) -> HarTexResult<()> {
    match {
        match interaction.clone() {
            Interaction::ApplicationCommand(command) => {
                match &*command.data.name {
                    // Global Administrator Only Plugin
                    "refroles" => {
                        emitter.emit(HarTexEvent::CommandExecuted(Box::new(CommandExecuted {
                            command: "refroles"
                        })));

                        let span =
                            tracing::trace_span!("interaction command handler: refroles command");

                        Refroles
                            .execute(
                                CommandContext {
                                    inner: Arc::new(CommandContextInner {
                                        http,
                                        cluster,
                                        interaction
                                    })
                                },
                                cache
                            )
                            .instrument(span)
                            .await
                    }
                    // Global Plugin
                    "about" => {
                        emitter.emit(HarTexEvent::CommandExecuted(Box::new(CommandExecuted {
                            command: "about"
                        })));

                        let span =
                            tracing::trace_span!("interaction command handler: about command");

                        About
                            .execute(
                                CommandContext {
                                    inner: Arc::new(CommandContextInner {
                                        http,
                                        cluster,
                                        interaction
                                    })
                                },
                                cache
                            )
                            .instrument(span)
                            .await
                    }
                    "ping" => {
                        emitter.emit(HarTexEvent::CommandExecuted(Box::new(CommandExecuted {
                            command: "ping"
                        })));

                        let span =
                            tracing::trace_span!("interaction command handler: ping command");

                        Ping.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        )
                        .instrument(span)
                        .await
                    }
                    "source" => {
                        emitter.emit(HarTexEvent::CommandExecuted(Box::new(CommandExecuted {
                            command: "source"
                        })));

                        let span =
                            tracing::trace_span!("interaction command handler: source command");

                        Source
                            .execute(
                                CommandContext {
                                    inner: Arc::new(CommandContextInner {
                                        http,
                                        cluster,
                                        interaction
                                    })
                                },
                                cache
                            )
                            .instrument(span)
                            .await
                    }
                    "team" => {
                        emitter.emit(HarTexEvent::CommandExecuted(Box::new(CommandExecuted {
                            command: "team"
                        })));

                        let span =
                            tracing::trace_span!("interaction command handler: team command");

                        Team.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        )
                        .instrument(span)
                        .await
                    }
                    // Information Plugin
                    "guildinfo" => {
                        emitter.emit(HarTexEvent::CommandExecuted(Box::new(CommandExecuted {
                            command: "guildinfo"
                        })));

                        let span =
                            tracing::trace_span!("interaction command handler: guildinfo command");

                        Guildinfo
                            .execute(
                                CommandContext {
                                    inner: Arc::new(CommandContextInner {
                                        http,
                                        cluster,
                                        interaction
                                    })
                                },
                                cache
                            )
                            .instrument(span)
                            .await
                    }
                    "userinfo" => {
                        emitter.emit(HarTexEvent::CommandExecuted(Box::new(CommandExecuted {
                            command: "userinfo"
                        })));

                        Userinfo
                            .execute(
                                CommandContext {
                                    inner: Arc::new(CommandContextInner {
                                        http,
                                        cluster,
                                        interaction
                                    })
                                },
                                cache
                            )
                            .await
                    }
                    _ => Ok(())
                }
            }
            _ => Ok(())
        }
    } {
        Ok(_) => (),
        Err(error) => {
            tracing::error!("failed to handle interaction due to an error: {error:?}");
        }
    }

    Ok(())
}
