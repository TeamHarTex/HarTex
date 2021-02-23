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

use twilight_model::{
    id::{
        ApplicationId,
        EmojiId,
        GuildId,
        UserId
    },
    oauth::{
        id::TeamId
    }
};

use crate::system::{
    twilight_id_extensions::IntoInnerU64,
    SystemError,
    SystemResult
};

use crate::utilities::{
    constants::content_distribution_network_base_url
};

crate struct ContentDistributionNetwork;

impl ContentDistributionNetwork {
    crate fn custom_emoji(emoji_id: EmojiId, animated: bool) -> SystemResult<String> {
        Ok(
            format!(
                "{}emojis/{}.{}?v=1",
                content_distribution_network_base_url(),
                emoji_id.into_inner_u64(),
                if animated { "gif" } else { "png" },
            )
        )
    }

    crate fn guild_icon(guild_id: GuildId, guild_icon: String, animated: bool) -> SystemResult<String> {
         if animated && guild_icon.starts_with("a_") {
            Ok(
                format!(
                    "{}icons/{}/{}.{}?v=1",
                    content_distribution_network_base_url(),
                    guild_id.into_inner_u64(),
                    guild_icon,
                    "gif"
                )
            )
        }
        else if !animated && !guild_icon.starts_with("a_") {
            Ok(
                format!(
                    "{}icons/{}/{}.{}?v=1",
                    content_distribution_network_base_url(),
                    guild_id.into_inner_u64(),
                    guild_icon,
                    "png"
                )
            )
        }
        else {
            Err(box SystemError("Both guild_icon and animated must be true or false to pass this.".to_string()))
        }
    }

    crate fn guild_splash(guild_id: GuildId, guild_splash: String) -> SystemResult<String> {
        Ok(
            format!(
                "{}splashes/{}.{}?v=1",
                content_distribution_network_base_url(),
                guild_id.into_inner_u64(),
                guild_splash
            )
        )
    }

    crate fn guild_discovery_splash(guild_id: GuildId, guild_discovery_splash: String) -> SystemResult<String> {
        Ok(
            format!(
                "{}discovery-splashes/{}.{}?v=1",
                content_distribution_network_base_url(),
                guild_id.into_inner_u64(),
                guild_discovery_splash
            )
        )
    }

    crate fn guild_banner(guild_id: GuildId, guild_banner: String) -> SystemResult<String> {
        Ok(
            format!(
                "{}banners/{}.{}?v=1",
                content_distribution_network_base_url(),
                guild_id.into_inner_u64(),
                guild_banner
            )
        )
    }

    crate fn default_user_avatar(user_discriminator: u16) -> SystemResult<String> {
        Ok(
            format!(
                "{}embed/avatars/{}.png?v=1",
                content_distribution_network_base_url(),
                user_discriminator % 5
            )
        )
    }

    crate fn user_avatar(user_id: UserId, user_avatar: String, animated: bool) -> SystemResult<String> {
         if animated && user_avatar.starts_with("a_") {
            Ok(
                format!(
                    "{}avatars/{}/{}.{}?v=1",
                    content_distribution_network_base_url(),
                    user_id.into_inner_u64(),
                    user_avatar,
                    "gif"
                )
            )
        }
        else if !animated && !user_avatar.starts_with("a_") {
            Ok(
                format!(
                    "{}avatars/{}/{}.{}?v=1",
                    content_distribution_network_base_url(),
                    user_id.into_inner_u64(),
                    user_avatar,
                    "png"
                )
            )
        }
        else {
            Err(box SystemError("Both guild_icon and animated must be true or false to pass this.".to_string()))
        }
    }

    crate fn application_icon(application_id: ApplicationId, icon: String) -> SystemResult<String> {
        Ok(
            format!(
                "{}app-icons/{}/{}.{}?v=1",
                content_distribution_network_base_url(),
                application_id.into_inner_u64(),
                icon,
                "png"
            )
        )
    }

    crate fn application_asset(application_id: ApplicationId, asset_id: String) -> SystemResult<String> {
        Ok(
            format!(
                "{}app-assets/{}/{}.{}?v=1",
                content_distribution_network_base_url(),
                application_id.into_inner_u64(),
                asset_id,
                "png"
            )
        )
    }

    crate fn achievement_icon(application_id: ApplicationId, achievement_id: String, icon: String) -> SystemResult<String> {
        Ok(
            format!(
                "{}app-assets/{}/achievements/{}/icons/{}.{}?v=1",
                content_distribution_network_base_url(),
                application_id.into_inner_u64(),
                achievement_id,
                icon,
                "png"
            )
        )
    }

    crate fn team_icon(team_id: TeamId, team_icon: String) -> SystemResult<String> {
        Ok(
            format!(
                "{}team-icons/{}/{}.{}?v=1",
                content_distribution_network_base_url(),
                team_id.into_inner_u64(),
                team_icon,
                "png"
            )
        )
    }
}
