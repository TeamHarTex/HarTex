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

//! # The `relations` Module
//!
//! This module contains helper functions for relation manipulations.

use crate::{
    backend::Backend,
    entity::Entity,
    repository::{
        GetEntityFuture,
        Repository
    }
};

pub fn map<
    'a,
    B: Backend,
    F: FnOnce(LE) -> FE::Id + Send + 'a,
    LE: Entity + 'a,
    FE: Entity + 'a,
    LR: Repository<LE, B> + Send + 'a,
    FR: Repository<FE, B> + Send + Sync + 'a
>(
    local_repo: LR,
    foreign_repo: FR,
    local_entity_id: LE::Id,
    f: F
) -> GetEntityFuture<'a, FE, B::Error>
where
    B::Error: Send
{
    Box::pin(async move {
        let future = local_repo.entity(local_entity_id);

        let foreign_entity_id = if let Some(entity) = future.await? {
            f(entity)
        }
        else {
            return Ok(None);
        };

        foreign_repo.entity(foreign_entity_id).await
    })
}
