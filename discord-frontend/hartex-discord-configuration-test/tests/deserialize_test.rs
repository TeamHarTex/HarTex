/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::BTreeMap;

use hartex_discord_configuration_manifest::appearance::Appearance;
use hartex_discord_configuration_manifest::Configuration;
use hartex_discord_configuration_manifest::dashboard::Dashboard;
use hartex_discord_configuration_manifest::ranks::RankPermission;
use hartex_discord_configuration_manifest::ranks::Ranks;
use hartex_discord_configuration_manifest::deserialize_config;

#[test]
pub fn deserialize_test() {
    let string = include_str!("../../hartex-discord-configuration-sample/configuration.hcl");
    let deserialized = deserialize_config(string).expect("deserialization failed");

    let expected = Configuration {
        version: 20,
        dashboard: Dashboard {
            admins: vec!["1000000000000000".to_string(), "1000000000000001".to_string()],
            editors: Some(vec!["1000000000000002".to_string(), "1000000000000003".to_string()]),
            viewers: Some(vec!["1000000000000004".to_string(), "1000000000000005".to_string()])
        },
        appearance: Appearance {
            nickname: Some("HarTex Nightly".to_string()),
            role_colour: Some(3285852160),
        },
        ranks: Ranks {
            roles: {
                let mut map = BTreeMap::new();
                map.insert("1234567890987654".to_string(), RankPermission {
                    level: 100
                });

                map
            },
            users: {
                let mut map = BTreeMap::new();
                map.insert("1000000000000000".to_string(), RankPermission {
                    level: 100
                });

                map
            },
        }
    };

    pretty_assertions::assert_eq!(deserialized, expected);
}
