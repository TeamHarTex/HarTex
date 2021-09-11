//! # The `handler` Module
//!
//! This module defines the `EventHandler` struct, which defines various function handlers for
//! individual events.

use tokio::time;

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        gateway::Cluster,
        http::Client,
        model::gateway::{
            event::shard::Identifying,
            payload::{
                update_presence::UpdatePresence,
                GuildCreate,
                InteractionCreate,
                MessageCreate,
                Ready,
            },
            presence::{
                Activity,
                ActivityType,
                Status,
            }
        }
    },
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_dbmani::{
    guildconf::GetGuildConfig,
    whitelist::GetWhitelistedGuilds
};

use hartex_eventsys::emitter::EventEmitter;

use hartex_logging::Logger;

use hartex_model::payload::CommandExecuted;

use hartex_plugins::{
    global::{
        about::About,
        ping::Ping,
        source::Source,
        team::Team
    }
    // information::userinfo::Userinfo
};

use crate::commands;

/// # Struct `EventHandler`
///
/// This structure defines various function handlers for individual events.
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
    pub async fn guild_create(payload: Box<GuildCreate>, http: Client) -> HarTexResult<()> {
        let guild_id = payload.id;

        Logger::verbose(
            format!("joined a new guild with name `{}` with id {}; checking whether the guild is whitelisted", payload.name, guild_id.0),
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        let res = GetWhitelistedGuilds::default().await?;

        if !res.iter().any(|guild| {
            guild_id.0 == guild.GuildId
        }) {
            Logger::error(
                "guild is not whitelisted",
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            let guild = http.guild(guild_id).exec().await?.model().await?;

            Logger::verbose(
                "dming guild owner about the whitelist status",
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            let guild_owner = guild.owner_id;

            let user = http.user(guild_owner).exec().await?.model().await?;

            let dm_channel = http.create_private_channel(user.id).exec().await?.model().await?;
            let message = "Hey there! It looks like you added HarTex to your guild by the name of \"".to_string()
                + &guild.name + "\".\n\n"
                + "Unfortunately, your guild has not been whitelisted yet and the bot cannot be "
                + "invited to your guild until you apply for a whitelist and that the application is "
                + "accepted.\n\n"
                + "You may apply for a guild whitelist if your guild meets the following criteria, they include, but not limited to:\n"
                + " - guild member count of at least 250;"
                + " - be always abide by the Discord Terms of Service (<https://discord.com/terms>) and Community Guidelines (<https://discord.com/guidelines);"
                + " - how old is the guild and/or how active is it; and"
                + " - your experience level with TOML to configure the bot before using it.\n\n"
                + "You may join our Support Guild at <discord.gg/s8qjxZK> for more information, including the application link in which you may use"
                + "to apply for a whitelist application. Good luck!";

            http.create_message(dm_channel.id).content(&message)?.exec().await?;

            Logger::error(
                "leaving guild",
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            http.leave_guild(guild_id).exec().await?;

            return Err(HarTexError::Custom {
                message: String::from("guild is not whitelisted")
            });
        }

        Logger::info(
            "guild is whitelisted",
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        Ok(())
    }

    /// # Static Asynchronous Method `EventHandler::interaction_create`
    ///
    /// Handles the `InteractionCreate` event.
    ///
    /// ## Parameters
    /// - `payload`, type `Box<InteractionCreate>`: the `InteractionCreate` event payload
    /// - `http`, type `Client`: the Twilight HTTP client to pass to the command if the message is indeed a command
    pub async fn interaction_create(
        payload: Box<InteractionCreate>,
        http: Client,
        cluster: Cluster,
        cache: InMemoryCache
    ) -> HarTexResult<()> {
        crate::interactions::handle_interaction(payload.0, cache, http, cluster).await?;

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
    pub async fn message_create(
        _: Box<MessageCreate>,
        _: EventEmitter,
        _: InMemoryCache,
        _: Client,
        _: Cluster
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
    pub async fn ready(payload: Box<Ready>, cluster: Cluster, http: Client) -> HarTexResult<()> {
        let user = payload.user;

        Logger::info(
            format!(
                "{}#{} [id: {}] has successfully startup; using discord api v{}",
                user.name,
                user.discriminator,
                user.id,
                payload.version
            ),
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        for shard in cluster.shards() {
            let shard_id = shard.info()?.id();

            Logger::verbose(
                format!("registering presence for shard {}", shard_id),
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            match shard.command(
                &UpdatePresence::new(
                    vec![Activity {
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
                    }],
                    false,
                    None,
                    Status::Online
                )?
            ).await {
                Ok(()) => {
                    Logger::verbose(
                        format!("successfully set presence for shard {}", shard_id),
                        Some(module_path!()),
                        file!(),
                        line!(),
                        column!()
                    );
                },
                Err(error) => {
                    Logger::error(
                        format!("failed to set presence for shard {}: {}", shard_id, error),
                        Some(module_path!()),
                        file!(),
                        line!(),
                        column!()
                    );
                }
            }
        }

        commands::register_global_slash_commands(
            vec![
                // Global Plugin
                Box::new(About),
                Box::new(Ping),
                Box::new(Source),
                Box::new(Team)

                // Information Plugin
                // Box::new(Userinfo)
            ],
            http.clone()
        ).await?;

        for guild in http.current_user_guilds().exec().await?.models().await? {
            Logger::verbose(
                format!("changing nickname in guild {}", guild.name),
                Some(module_path!()),
                file!(),
                line!(),
                column!()
            );

            let config = GetGuildConfig::new(guild.id).await?;

            match http.update_current_user_nick(guild.id, &config.GuildConfiguration.nickname).exec().await {
                Err(error) => {
                    Logger::error(
                        format!("failed to change nickname: {}", error),
                        Some(module_path!()),
                        file!(),
                        line!(),
                        column!()
                    );
                }
                _ => ()
            };

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
    pub async fn shard_identifying(payload: Identifying) -> HarTexResult<()> {
        Logger::verbose(
            format!(
                "shard {} is identifying with the discord gateway",
                payload.shard_id
            ),
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        Ok(())
    }
}

// Custom Events
impl EventHandler {
    /// # Static Asynchronous Method `EventHandler::command_executed`
    ///
    /// Handles the `CommandExecuted` event.
    ///
    /// ## Parameters
    ///
    /// - `payload`, type `Box<CommandExecuted>`: the `CommandExecuted` event payload
    pub async fn command_executed(payload: Box<CommandExecuted>) -> HarTexResult<()> {
        Logger::info(
            format!("command `{}` is executed in guild {}", payload.command, payload.guild.name),
            Some(module_path!()),
            file!(),
            line!(),
            column!()
        );

        Ok(())
    }
}
