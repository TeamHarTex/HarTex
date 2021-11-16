//! # The `refroles` Module
//!
//! This module implements the `refroles` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
    context::CommandContext
};
use hartex_core::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        model::application::{
            callback::{
                CallbackData,
                InteractionResponse
            },
            interaction::Interaction
        }
    },
    error::{
        HarTexError,
        HarTexResult
    },
    logging::tracing
};
use hartex_dbmani::{
    guildconf::GetGuildConfig,
    whitelist::GetWhitelistedGuilds
};
use hartex_utils::FutureRetType;
use tokio::time;

use crate::PLUGIN_ENV;

/// # Struct `Refroles`
///
/// The `refroles` command.
pub struct Refroles;

impl Command for Refroles {
    fn name(&self) -> String {
        String::from("refroles")
    }

    fn description(&self) -> String {
        String::from("GlobAdminOnlyPlugin.RefrolesCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        cache: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_refroles_command(ctx, cache))
    }
}

/// # Asynchronous Function `execute_refroles_command`
///
/// Executes the `refroles` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
#[allow(clippy::missing_panics_doc)]
async fn execute_refroles_command(
    ctx: CommandContext,
    _: CloneableInMemoryCache
) -> HarTexResult<()> {
    let interaction = if let Interaction::ApplicationCommand(command) = ctx.interaction.clone() {
        command
    }
    else {
        tracing::error!("invalid interaction type: expected ApplicationCommand");

        return Err(HarTexError::Custom {
            message: String::from("invalid interaction type: expected ApplicationCommand")
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
                        ":x: This command can only be used in a guild."
                    )),
                    embeds: vec![],
                    flags: None,
                    tts: None
                })
            )
            .exec()
            .await?;
    }

    if interaction.member.unwrap().user.unwrap().id != PLUGIN_ENV.global_administrator_uid.unwrap()
    {
        ctx.http
            .interaction_callback(
                interaction.id,
                &interaction.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(String::from(":x: You are not the global administrator.")),
                    embeds: vec![],
                    flags: None,
                    tts: None
                })
            )
            .exec()
            .await?;
    }

    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(CallbackData {
                allowed_mentions: None,
                components: None,
                content: Some(String::from(
                    "Stage `1` of 1: obtaining whitelisted guilds..."
                )),
                embeds: vec![],
                flags: None,
                tts: None
            })
        )
        .exec()
        .await?;

    let guilds = GetWhitelistedGuilds::default().await?;

    for guild in guilds {
        let config = GetGuildConfig::new(guild.GuildId).await?;

        for access in config.DashboardAccess {
            ctx.http
                .add_guild_member_role(
                    PLUGIN_ENV.support_guild_gid.unwrap(),
                    access.userId,
                    PLUGIN_ENV.hartex_user_rid.unwrap()
                )
                .exec()
                .await?;

            time::sleep(time::Duration::from_secs(1)).await;
        }
    }

    Ok(())
}
