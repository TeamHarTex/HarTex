/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `guildconf` Module
//!
//! This module contains configuration models specifically for guild-specific configuration.

use serde::{
    de,
    Deserialize
};

pub mod locale;
pub mod tz;

/// # Struct `GuildConfiguration`
///
/// Represents guild-specific configuration.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GuildConfiguration {
    #[serde(default)]
    pub nickname: Option<String>,
    #[serde(
        default = "default_timezone",
        deserialize_with = "deserialize_timezone"
    )]
    pub timezone: tz::Timezone,
    #[serde(default = "default_dm_cant_use_cmd")]
    pub dmCannotUseCommand: bool,
    #[serde(default = "default_locale", deserialize_with = "deserialize_locale")]
    pub locale: locale::Locale
}

fn default_locale() -> locale::Locale {
    locale::Locale::EnGb
}

fn default_timezone() -> tz::Timezone {
    tz::Timezone::UTC
}

fn default_dm_cant_use_cmd() -> bool {
    true
}

fn deserialize_locale<'deserialize, D>(deserializer: D) -> Result<locale::Locale, D::Error>
where
    D: de::Deserializer<'deserialize> {
    deserializer.deserialize_str(locale::GuildConfigLocaleDeserializerRefStrVisitor)
}

fn deserialize_timezone<'deserialize, D>(deserializer: D) -> Result<tz::Timezone, D::Error>
where
    D: de::Deserializer<'deserialize> {
    deserializer.deserialize_str(tz::GuildConfigTimezoneDeserializerRefStrVisitor)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use serde_test::Token;

    use super::{
        locale,
        tz,
        Deserialize,
        GuildConfiguration
    };

    const _: fn() = || {
        fn static_assert_impl_all<
            'deserialize,
            T: ?Sized + Clone + Debug + Deserialize<'deserialize> + PartialEq
        >() {
        }

        static_assert_impl_all::<GuildConfiguration>();
    };

    #[test]
    fn test_guildconf_de() {
        serde_test::assert_de_tokens(
            &GuildConfiguration {
                nickname: Some(String::from("HarTex")),
                timezone: tz::Timezone::UTC,
                dmCannotUseCommand: true,
                locale: locale::Locale::EnGb
            },
            &[
                Token::Struct {
                    name: "GuildConfiguration",
                    len: 3
                },
                Token::Str("nickname"),
                Token::Some,
                Token::Str("HarTex"),
                Token::Str("timezone"),
                Token::Str("UTC"),
                Token::Str("dmCannotUseCommand"),
                Token::Bool(true),
                Token::Str("locale"),
                Token::Str("en_GB"),
                Token::StructEnd
            ]
        );
    }
}
