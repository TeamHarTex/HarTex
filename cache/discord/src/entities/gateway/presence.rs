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

//! The gateway presence entity.

use hartex_base::discord::model::{
    gateway::presence::{Activity, ClientStatus, Presence, Status, UserOrId},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use hartex_cache_base::entity::Entity;

/// A presence entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct PresenceEntity {
    activities: Vec<Activity>,
    client_status: ClientStatus,
    guild_id: Id<GuildMarker>,
    status: Status,
    user_id: Id<UserMarker>,
}

impl PresenceEntity {
    #[must_use]
    pub fn activities(&self) -> Vec<Activity> {
        self.activities.clone()
    }

    #[must_use]
    pub fn client_status(&self) -> ClientStatus {
        self.client_status.clone()
    }

    #[must_use]
    pub fn guild_id(&self) -> Id<GuildMarker> {
        self.guild_id
    }

    #[must_use]
    pub fn status(&self) -> Status {
        self.status
    }

    #[must_use]
    pub fn user_id(&self) -> Id<UserMarker> {
        self.user_id
    }
}

impl Entity for PresenceEntity {
    type Id = (Id<GuildMarker>, Id<UserMarker>);

    fn id(&self) -> Self::Id {
        (self.guild_id, self.user_id)
    }
}

impl From<Presence> for PresenceEntity {
    fn from(presence: Presence) -> Self {
        #[rustfmt::skip]
        let user_id = match presence.user {
            UserOrId::User(user) => user.id,
            UserOrId::UserId { id } => id
        };

        Self {
            activities: presence.activities,
            client_status: presence.client_status,
            guild_id: presence.guild_id,
            status: presence.status,
            user_id,
        }
    }
}
