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
            StickerBannerAssetId,
            StickerFormatType,
            StickerId,
            StickerPack,
            StickerPackId,
            StickerPackSkuId,
            StickerType
        },
        id::{
            GuildId,
            UserId
        }
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
    pub fn sort_value(&self) -> Option<u64> {
        self.sort_value
    }

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
    type Id = (Option<StickerPackId>, StickerId);

    fn id(&self) -> Self::Id {
        (self.pack_id, self.id)
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

/// # Struct `StickerEntity`
///
/// A sticker pack entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct StickerPackEntity {
    banner_asset_id: Option<StickerBannerAssetId>,
    cover_sticker_id: Option<StickerId>,
    description: String,
    id: StickerPackId,
    name: String,
    sku_id: StickerPackSkuId,
    sticker_ids: Vec<StickerId>
}

impl StickerPackEntity {
    #[must_use]
    pub fn banner_asset_id(&self) -> Option<StickerBannerAssetId> {
        self.banner_asset_id
    }

    #[must_use]
    pub fn cover_sticker_id(&self) -> Option<StickerId> {
        self.cover_sticker_id
    }

    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[must_use]
    pub fn sku_id(&self) -> StickerPackSkuId {
        self.sku_id
    }

    #[must_use]
    pub fn sticker_ids(&self) -> Vec<StickerId> {
        self.sticker_ids.clone()
    }
}

impl Entity for StickerPackEntity {
    type Id = (StickerPackId, StickerPackSkuId);

    fn id(&self) -> Self::Id {
        (self.id, self.sku_id)
    }
}

impl From<StickerPack> for StickerPackEntity {
    fn from(sticker_pack: StickerPack) -> Self {
        let sticker_ids = sticker_pack.stickers.iter().map(|sticker| sticker.id).collect();

        Self {
            banner_asset_id: sticker_pack.banner_asset_id,
            cover_sticker_id: sticker_pack.cover_sticker_id,
            description: sticker_pack.description,
            id: sticker_pack.id,
            name: sticker_pack.name,
            sku_id: sticker_pack.sku_id,
            sticker_ids
        }
    }
}
