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
/// - `cluster`: the gateway cluster
///
/// ## Errors
///
/// Interaction-related errors.
#[allow(clippy::too_many_lines)]
pub async fn handle_interaction(
    interaction: Interaction,
    cache: CloneableInMemoryCache,
    http: CloneableClient,
    cluster: CloneableCluster
) -> HarTexResult<()> {
    let span = tracing::trace_span!("interaction handler");

    match {
        match interaction.clone() {
            Interaction::ApplicationCommand(command) => {
                match &*command.data.name {
                    // Global Administrator Only Plugin
                    "refroles" => {
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
                            .await
                    }
                    // Global Plugin
                    "about" => {
                        span.in_scope(|| {
                            tracing::trace!(
                                "interaction command identified - `about`; invoking command handler"
                            );
                        });

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
                        span.in_scope(|| {
                            tracing::trace!(
                                "interaction command identified - `ping`; invoking command handler"
                            );
                        });

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
                        span.in_scope(|| {
                            tracing::trace!(
                                "interaction command identified - `source`; invoking command handler"
                            );
                        });

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
                        span.in_scope(|| {
                            tracing::trace!(
                                "interaction command identified - `team`; invoking command handler"
                            );
                        });

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
                        span.in_scope(|| {
                            tracing::trace!(
                                "interaction command identified - `guildinfo`; invoking command handler"
                            );
                        });

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
                        span.in_scope(|| {
                            tracing::trace!(
                                "interaction command identified - `userinfo`; invoking command handler"
                            );
                        });

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
