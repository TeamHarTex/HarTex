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

// This file is a sample configuration for the per-guild configuration of HarTex.
// This sample may change as the data structures and manifest evolves.

// Version of manifest, automatically generated.
// DO NOT EDIT: will break functionality in some form or another!
version = "1"

// Dashboard access configurations.
dashboard {
    // Admins of the server, they can add people to the configuration editor
    // Array of user IDs as strings.
    admins = ["1000000000000000", "1000000000000001"]
    // Editors of the server, they can edit the configuration but not add people to the configuration editor.
    // Array of user IDs as strings.
    editors = ["1000000000000002", "1000000000000003"]
    // Viewers of the server, they can only view the configuration.
    // Array of user IDs as strings.
    viewers = ["1000000000000004", "1000000000000005"]
}

// Appearance of HarTex in the server.
appearance {
    // Nickname of the bot user in the server.
    // A string.
    nickname = "HarTex Nightly"
}
