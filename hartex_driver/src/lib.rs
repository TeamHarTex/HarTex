//! # `hartex_driver` - The "Main Function" of HarTex Discord bot
//!
//! This `hartex_driver` crate contains effectively the "main function" of the bot as well as some
//! "moving pieces" that are required for the bot to work.

use std::{
    env,
    process
};

use hartex_core::{
    discord::{
        gateway::{
            cluster::{
                Cluster,
                ShardScheme,
            },
            Intents,
        },
        http::Client,
        cache_inmemory::{
            InMemoryCache,
            ResourceType
        }
    },
    error::HarTexResult
};

use hartex_logging::Logger;

/// # Asynchronous Function `hartex_main`
///
/// This is the main entry point of HarTex Discord Bot.
///
/// ## Return Type
/// `HarTexResult<()>`
pub async fn hartex_main() -> HarTexResult<()> {
    // loads the .env file to obtain environment variables
    dotenv::dotenv().ok();

    Logger::verbose("loaded environment variables", Some(module_path!()));

    // obtains the token from the environment variables
    let token = match env::var("HARTEX_TOKEN") {
        Ok(token) => token,
        Err(var_error) => {
            Logger::error(
                format!(
                    "could not retrieve the bot token from the environment due to an error: {}",
                    var_error
                ),
                Some(module_path!())
            );

            process::exit(-1)
        }
    };

    Logger::verbose("successfully retrieved bot token", Some(module_path!()));

    let shard_scheme = ShardScheme::Auto;

    Logger::verbose("building bot cluster", Some(module_path!()));
    Logger::verbose("registering gateway intents [all]", Some(module_path!()));

    let intents = Intents::all();

    let (cluster, mut events) = Cluster::builder(&token, intents)
        .shard_scheme(shard_scheme)
        .build()
        .await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await
    });

    Logger::verbose("building http client", Some(module_path!()));

    let http = Client::new(&token);

    Logger::verbose("building in-memory cache", Some(module_path!()));

    let resource_types = ResourceType::all();
    let cache = InMemoryCache::builder()
        .resource_types(resource_types)
        .build();

    Logger::verbose("registering ctrl-c handler", Some(module_path!()));

    ctrlc::set_handler(|| {
        Logger::warn("ctrl-c signal received; terminating process", Some(module_path!()));

        process::exit(0)
    })?;

    Ok(())
}
