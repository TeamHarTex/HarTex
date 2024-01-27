/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

use expect_test::expect;
use hartex_discord_configuration_luart::evaluate_config;

const SAMPLE_CONFIG: &'static str = r#"return {
    dashboard = {
        admins = { "1000000000000000", "1000000000000001" },
        editors = { "1000000000000002", "1000000000000003" },
        viewers = { "1000000000000004", "1000000000000005" }
    },

    appearance = {
        nickname = "HarTex Nightly",
        colour = hartexconf.colour.rgb(0x768EE5)
    },

    permissions = {
        roles = {
            ["1234567890987654"] = 100
        },

        users = {
            ["1000000000000000"] = 100
        }
    },

    plugins = {
        utilities = {
            enabled = true
        }
    }
}
"#;

#[test]
pub fn parse_test() {
    let config = evaluate_config(SAMPLE_CONFIG).unwrap();
    let expected = expect![
        r#"
Configuration {
    appearance: Some(
        Appearance {
            colour: Some(
                7769829,
            ),
            nickname: Some(
                "HarTex Nightly",
            ),
        },
    ),
    dashboard: Dashboard {
        admins: [
            "1000000000000000",
            "1000000000000001",
        ],
        editors: Some(
            [
                "1000000000000002",
                "1000000000000003",
            ],
        ),
        viewers: Some(
            [
                "1000000000000004",
                "1000000000000005",
            ],
        ),
    },
    plugins: Some(
        Plugins {
            utilities: Some(
                UtilitiesPlugin {
                    enabled: true,
                },
            ),
        },
    ),
}
"#
    ];

    expected.assert_debug_eq(&config);
}
