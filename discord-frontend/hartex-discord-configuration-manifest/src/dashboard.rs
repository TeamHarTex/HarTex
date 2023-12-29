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

use serde::Deserialize;
use serde::Serialize;

/// Dashboard access configurations.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Dashboard {
    /// Admins of the server, they can add people to the configuration editor.
    pub admins: Vec<String>,
    /// Editors of the server, they can edit the configuration but not add people to the
    /// configuration editor.
    pub editors: Option<Vec<String>>,
    /// Viewers of the server, they can only view the configuration.
    pub viewers: Option<Vec<String>>,
}
