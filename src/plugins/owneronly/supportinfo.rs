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

use std::{
    future::Future,
    pin::Pin
};

use twilight_cache_inmemory::InMemoryCache;

use twilight_embed_builder::{
    EmbedBuilder,
    EmbedFieldBuilder
};

use crate::command_system::{
    parser::{
        Arguments
    },
    Command,
    CommandContext,
    PrecommandCheckParameters
};

use crate::system::SystemResult;

use crate::utilities::FutureResult;

crate struct SupportinfoCommand;

impl Command for SupportinfoCommand {
    fn fully_qualified_name(&self) -> String {
        String::from("support-info")
    }

    fn execute_command<'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                            _arguments: Arguments<'asynchronous_trait>, _cache: InMemoryCache)
        -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> {
        Box::pin(owneronly_supportinfo_command(ctx))
    }

    fn precommand_checks<'asynchronous_trait, C: 'asynchronous_trait>(ctx: CommandContext<'asynchronous_trait>,
                                                                      params: PrecommandCheckParameters, checks: Box<[C]>)
                                                                      -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> where
        C: Fn(CommandContext<'asynchronous_trait>, PrecommandCheckParameters)
            -> Pin<Box<dyn Future<Output=SystemResult<()>> + Send + 'asynchronous_trait>> + Send + Sync {
        Box::pin(
            async move {
                for check in checks.iter() {
                    if let Err(error) = check(ctx.clone(), params.clone()).await {
                        return Err(error);
                    }
                    else {
                        continue;
                    }
                }

                Ok(())
            }
        )
    }
}

async fn owneronly_supportinfo_command(ctx: CommandContext<'_>) -> SystemResult<()> {
    let channel_id = ctx.message.channel_id;

    ctx.http_client.delete_message(channel_id, ctx.message.id).await?;

    let first_embed = EmbedBuilder::new()
        .title("**__Information About HarTex__**")
        .description(
            "HarTex is built and optimized for infractions and administration, for Discord guilds. It is meant to be".to_owned()
                + " flexible and highly configurable by guild administrators and moderators to fit the specific needs"
                + " of their guilds.\n\nPlease note that the bot adopts a whitelist system. Meaning of you want to"
                + " invite the bot to your guild, you need to apply for a whitelist beforehand. If the bot joins a"
                + " guild that is not whitelisted, it leaves it immediately. ")
        .field(
            EmbedFieldBuilder::new(
                "**__Bot Support__**",
                "Users with the role `Verified HarTex User` can access the <#791588908369707038> channel to".to_owned()
                    + " receive support for the bot. In order to get the role, you need to be able to access the web"
                    + " dashboard, to the guild configuration, of a guild.\n\nGuild owners with the bot in their guild"
                    + " will have an additional `HarTex Guild Owner` role which grants access to the"
                    + " <#791588967874691072>."))
        .color(0x03_BE_FC)
        .build()?;

    let second_embed = EmbedBuilder::new()
        .title("**__Minimum Whitelist Requirements__**")
        .description(
            "Whitelist applications are reviewed at a case-by-case basis. The following are the minimum requirements".to_owned()
                + " for a whitelist application. If your guild does not meet the following requirements, do not"
                + " apply. We preserve the right to approve or deny any whitelist application at my discretion.")
        .field(EmbedFieldBuilder::new("**__Member Count__**",
                                      "Your guild should have at least **250 - 500** members."))
        .field(EmbedFieldBuilder::new("**__Stay Abide By The Discord ToS__**",
                                      "Your guild should be always abide by the".to_owned()
                                          + " [Discord ToS](https://discord.com/terms) and"
                                          + " [Community Guidelines](https://discord.com/guidelines)."))
        .field(EmbedFieldBuilder::new("**__Communication Protocol__**",
                                      "At least one member of your guild, specifically needs to be on the".to_owned()
                                          + " `Dashboard` section of your guild configuration, needs to maintain"
                                          + " membership in this guild for contact purposes."))
        .field(EmbedFieldBuilder::new("**__Other Things I May Consider, But Not Limited To:__**",
                                      "- Guild Subject / Topic / Community;\n- Age and Activity of the Guild;".to_owned()
                                          + " and\n- Application Strength."))
        .field(EmbedFieldBuilder::new("**__Whitelist Application__**",
                                      "If your guild meets the above criteria, you may apply for a whitelist".to_owned()
                                          + " here: *Link Not Available Yet*"))
        .color(0x03_BE_FC)
        .build()?;

    let third_embed = EmbedBuilder::new()
        .title("**__Useful Links__**")
        .field(EmbedFieldBuilder::new("**__Bot Dashboard__**", "*Coming Soon...*"))
        .field(EmbedFieldBuilder::new("**__Configuration Documentation__**", "*Coming Soon...*"))
        .field(EmbedFieldBuilder::new("**__Basic Configuration Template__**", "*Coming Soon...*"))
        .field(EmbedFieldBuilder::new("**__Bot Invitation__**",
                                      "Once you receive a confirmation DM from either the bot or one of the".to_owned()
                                          + " developers, you may invite the bot with"
                                          + " [this link](https://discordapp.com/api/oauth2/authorize?client_id=688970404391550987&permissions=470150390&scope=bot)."))
        .color(0x03_BE_FC)
        .build()?;

    ctx.http_client.create_message(channel_id).embed(first_embed)?.await?;
    ctx.http_client.create_message(channel_id).embed(second_embed)?.await?;
    ctx.http_client.create_message(channel_id).embed(third_embed)?.await?;

    Ok(())
}
