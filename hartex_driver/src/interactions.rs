use std::sync::Arc;

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        gateway::Cluster,
        http::Client,
        model::application::interaction::Interaction
    },
    error::HarTexResult
};

use hartex_cmdsys::{
    command::SlashCommand,
    context::{
        CommandContext,
        CommandContextInner
    }
};

use hartex_logging::Logger;

use hartex_plugins::global::{
    about::About,
    ping::Ping,
    source::Source,
    team::Team
};

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
                    "about" => {
                        About.execute_slash_command(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    message: None,
                                    cluster,
                                    interaction: Some(interaction)
                                })
                            },
                            cache
                        ).await
                    }
                    "ping" => {
                        Ping.execute_slash_command(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    message: None,
                                    cluster,
                                    interaction: Some(interaction)
                                })
                            },
                            cache
                        ).await
                    }
                    "source" => {
                        Source.execute_slash_command(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    message: None,
                                    cluster,
                                    interaction: Some(interaction)
                                })
                            },
                            cache
                        ).await
                    },
                    "team" => {
                        Team.execute_slash_command(
                            CommandContext {
                                inner: Arc::new(CommandContextInner {
                                    http,
                                    message: None,
                                    cluster,
                                    interaction: Some(interaction)
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
            Logger::error(
                format!("failed to handle interaction due to an error: {:?}", error),
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );
        }
    }

    Ok(())
}