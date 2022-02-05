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

//! A command context used in commands.

use std::{ops::Deref, sync::Arc};

use hartex_base::discord::{
    gateway::CloneableCluster, http::CloneableClient, model::application::interaction::Interaction,
};

/// The command context used for command invocation.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct CommandContext {
    pub inner: Arc<CommandContextInner>,
}

/// The inner structure for `CommandContext`.
#[derive(Clone)]
pub struct CommandContextInner {
    pub http: CloneableClient,
    pub cluster: CloneableCluster,
    pub interaction: Interaction,
}

impl Deref for CommandContext {
    type Target = CommandContextInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
