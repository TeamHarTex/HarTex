//! # The `userinfo` Module
//!
//! This module implements the `userinfo` command

use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
    context::CommandContext
};

use hartex_core::{
    discord::{
        cache_inmemory::InMemoryCache,
        embed_builder::{
            EmbedAuthorBuilder,
            EmbedBuilder,
            EmbedFieldBuilder,
            ImageSource
        },
        model::{
            application::{
                callback::{
                    CallbackData,
                    InteractionResponse,
                },
                command::{
                    BaseCommandOptionData,
                    CommandOption,
                },
                interaction::{
                    application_command::CommandDataOption,
                    Interaction,
                },
            },
            id::UserId
        }
    },
    error::{
        HarTexError,
        HarTexResult
    }
};

use hartex_utils::{
    cdn::{
        Cdn,
        CdnResourceFormat
    },
    FutureRetType
};

/// # Struct `Userinfo`
///
/// The `userinfo` command.
pub struct Userinfo;

impl Command for Userinfo {
    fn name(&self) -> String {
        String::from("userinfo")
    }

    fn description(&self) -> String {
        String::from("InformationPlugin.UserinfoCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(&self, ctx: CommandContext, cache: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_userinfo_command(ctx, cache))
    }

    fn optional_cmdopts(&self) -> Vec<CommandOption> {
        vec![
            CommandOption::Mentionable(BaseCommandOptionData {
                description: String::from("(optional) the user to query the information"),
                name: String::from("user"),
                required: false
            })
        ]
    }
}

/// # Asynchronous Function `execute_userinfo_command`
///
/// Executes the `userinfo` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
async fn execute_userinfo_command(ctx: CommandContext, cache: InMemoryCache) -> HarTexResult<()> {
    let interaction = match ctx.interaction.clone() {
        Interaction::ApplicationCommand(command) => command,
        _ => return Err(
            HarTexError::Custom {
                message: String::from("invalid interaction type: expected ApplicationCommand")
            }
        )
    };

    if interaction.guild_id.is_none() || interaction.user.is_some() {
        ctx.http
            .interaction_callback(
                interaction.id,
                &interaction.token,
                &InteractionResponse::ChannelMessageWithSource(
                    CallbackData {
                        allowed_mentions: None,
                        components: None,
                        content: Some(String::from(":x: This command can only be used in a guild.")),
                        embeds: vec![],
                        flags: None,
                        tts: None
                    }
                )
            )
            .exec()
            .await?;
    }

    let options = interaction.data.options;
    let user = if options.is_empty() {
        // unwrapping here is fine as it is now ensured that the interaction is sent from a guild,
        // not in a user DM (which is the case when interaction.member is None)
        interaction.member.unwrap().user.unwrap()
    }
    else {
        // unwrapping here is fine because the command only accepts a "user" parameter and is
        // asserted to be of type "String"; therefore, the parameter must exist if the options vec
        // is not empty and must be with the name "user" and of type "String"
        let user_option = options
            .into_iter()
            .find(|option| option.name() == "user" && option.kind() == "String")
            .unwrap();
        let value = if let CommandDataOption::String { value, ..} = user_option {
            value
        }
        else {
            unreachable!("unexpected parameter type")
        };

        ctx.http
            .user(UserId::from(value.parse::<u64>().unwrap()))
            .exec()
            .await?
            .model()
            .await?
    };
    let member = ctx.http
        // it is ok to unwrap here because it is already checked that the interaction is sent from
        // a guild (which its id should never be None)
        .guild_member(interaction.guild_id.unwrap(), user.id)
        .exec()
        .await?
        .model()
        .await?;

    let avatar_url = if let Some(hash) = user.avatar {
        let format = if hash.starts_with("a_") {
            CdnResourceFormat::GIF
        }
        else {
            CdnResourceFormat::PNG
        };

        Cdn::user_avatar(user.id, hash, format)
    }
    else {
        Cdn::default_user_avatar(user.discriminator.parse().unwrap())
    };

    let embed = EmbedBuilder::new()
        .author(EmbedAuthorBuilder::new()
            .name(user.name)
            .icon_url(ImageSource::url(avatar_url)?)
        )
        .color(0x03BEFC);

    Ok(())
}