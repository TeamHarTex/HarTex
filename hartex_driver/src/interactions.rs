use std::sync::Arc;

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        gateway::Cluster,
        http::Client,
        model::application::interaction::Interaction
    },
    error::HarTexResult,
    logging::tracing
};

use hartex_cmdsys::{
    command::Command,
    context::{
        CommandContext,
        CommandContextInner
    }
};


use hartex_plugins::{
    globadmin_only::refroles::Refroles,
    global::{
        about::About,
        ping::Ping,
        source::Source,
        team::Team,
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
pub async fn handle_interaction(
    interaction: Interaction,
    cache: InMemoryCache,
    http: Client,
    cluster: Cluster
) -> HarTexResult<()> {
    match {
        match interaction.clone() {
            Interaction::ApplicationCommand(command) => {
                match &*command.data.name {
                    // Global Administrator Only Plugin
                    "refroles" => {
                        Refroles.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }

                    // Global Plugin
                    "about" => {
                        About.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }
                    "ping" => {
                        Ping.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }
                    "source" => {
                        Source.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    },
                    "team" => {
                        Team.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }

                    // Information Plugin
                    "guildinfo" => {
                        Guildinfo.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }
                    "userinfo" => {
                        Userinfo.execute(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    cluster,
                                    interaction
                                })
                            },
                            cache
                        ).await
                    }
                    _ => Ok(())
                }
            }
            _ => Ok(())
        }
    } {
        Ok(_) => (),
        Err(error) => {
            tracing::error!(
                "failed to handle interaction due to an error: {error:?}"
            );
        }
    }

    Ok(())
}