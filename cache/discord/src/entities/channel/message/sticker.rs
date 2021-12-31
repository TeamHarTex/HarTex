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

//! # The `sticker` Module
//!
//! This module implements the channel message sticker entity.

use hartex_base::{
    discord::model::{
        channel::message::sticker::{
            Sticker,
            StickerFormatType,
            StickerId,
            StickerPackId,
            StickerType
        },
        id::{
            GuildId,
            UserId
        },
    },
    stdext::prelude::*
};

use crate::entity::Entity;

/// # Struct `StickerEntity`
/// 
/// A message sticker entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct StickerEntity {
    available: bool,
    description: Option<String>,
    format_type: StickerFormatType,
    guild_id: Option<GuildId>,
    id: StickerId,
    kind: StickerType,
    name: String,
    pack_id: Option<StickerPackId>,
    sort_value: Option<u64>,
    tags: String,
    user_id: Option<UserId>
}

impl StickerEntity {
    #[must_use]
    pub fn available(&self) -> bool {
        self.available
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_refstr()
    }

    #[must_use]
    pub fn format_type(&self) -> StickerFormatType {
        self.format_type
    }

    #[must_use]
    pub fn guild_id(&self) -> Option<GuildId> {
        self.guild_id
    }

    #[must_use]
    pub fn kind(&self) -> StickerType {
        self.kind
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn pack_id(&self) -> Option<StickerPackId> {
        self.pack_id
    }

    #[must_use]
    pub fn sort_value(&self) -> Option<u64> { self.sort_value }

    #[must_use]
    pub fn tags(&self) -> &str {
        self.tags.as_ref()
    }

    #[must_use]
    pub fn user_id(&self) -> Option<UserId> {
        self.user_id
    }
}

impl Entity for StickerEntity {
    type Id = StickerId;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<Sticker> for StickerEntity {
    fn from(sticker: Sticker) -> Self {
        let user_id = sticker.user.map(|user| user.id);

        Self {
            available: sticker.available,
            description: sticker.description,
            format_type: sticker.format_type,
            guild_id: sticker.guild_id,
            id: sticker.id,
            kind: sticker.kind,
            name: sticker.name,
            pack_id: sticker.pack_id,
            sort_value: sticker.sort_value,
            tags: sticker.tags,
            user_id
        }
    }
}
