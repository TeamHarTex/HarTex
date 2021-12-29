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
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `role` Module
//!
//! This module contains the guild role repository trait.

use hartex_base::discord::model::id::RoleId;

use crate::{
    backend::Backend,
    entities::guild::{
        role::RoleEntity,
        GuildEntity
    },
    relations,
    repository::{
        GetEntityFuture,
        Repository
    }
};

/// # Trait `RoleRepository`
///
/// A repository containing Discord role objects.
#[allow(clippy::module_name_repetitions)]
pub trait RoleRepository<B: Backend>: Repository<RoleEntity, B> {
    /// # Trait Method `guild`
    ///
    /// Returns the guild this role is associated with.
    fn guild(&self, role_id: RoleId) -> GetEntityFuture<'_, GuildEntity, B::Error> {
        let backend = self.backend();

        relations::map_entity(backend.roles(), backend.guilds(), role_id, |role| {
            role.guild_id()
        })
    }
}
