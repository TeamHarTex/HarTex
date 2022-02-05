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

//! The channel message sticker entity.

use hartex_base::{
    discord::model::{
        channel::message::sticker::{Sticker, StickerFormatType, StickerPack, StickerType},
        id::{
            marker::{
                GuildMarker, StickerBannerAssetMarker, StickerMarker, StickerPackMarker,
                StickerPackSkuMarker, UserMarker,
            },
            Id,
        },
    },
    stdext::prelude::*,
};
use hartex_cache_base::entity::Entity;

/// A message sticker entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct StickerEntity {
    available: bool,
    description: Option<String>,
    format_type: StickerFormatType,
    guild_id: Option<Id<GuildMarker>>,
    id: Id<StickerMarker>,
    kind: StickerType,
    name: String,
    pack_id: Option<Id<StickerPackMarker>>,
    sort_value: Option<u64>,
    tags: String,
    user_id: Option<Id<UserMarker>>,
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
    pub fn guild_id(&self) -> Option<Id<GuildMarker>> {
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
    pub fn pack_id(&self) -> Option<Id<StickerPackMarker>> {
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
    pub fn user_id(&self) -> Option<Id<UserMarker>> {
        self.user_id
    }
}

impl Entity for StickerEntity {
    type Id = Id<StickerMarker>;

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
            user_id,
        }
    }
}

/// A sticker pack entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct StickerPackEntity {
    banner_asset_id: Option<Id<StickerBannerAssetMarker>>,
    cover_sticker_id: Option<Id<StickerMarker>>,
    description: String,
    id: Id<StickerPackMarker>,
    name: String,
    sku_id: Id<StickerPackSkuMarker>,
    sticker_ids: Vec<Id<StickerMarker>>,
}

impl StickerPackEntity {
    #[must_use]
    pub fn banner_asset_id(&self) -> Option<Id<StickerBannerAssetMarker>> {
        self.banner_asset_id
    }

    #[must_use]
    pub fn cover_sticker_id(&self) -> Option<Id<StickerMarker>> {
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
    pub fn sku_id(&self) -> Id<StickerPackSkuMarker> {
        self.sku_id
    }

    #[must_use]
    pub fn sticker_ids(&self) -> Vec<Id<StickerMarker>> {
        self.sticker_ids.clone()
    }
}

impl Entity for StickerPackEntity {
    type Id = Id<StickerPackMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<StickerPack> for StickerPackEntity {
    fn from(sticker_pack: StickerPack) -> Self {
        let sticker_ids = sticker_pack
            .stickers
            .iter()
            .map(|sticker| sticker.id)
            .collect();

        Self {
            banner_asset_id: sticker_pack.banner_asset_id,
            cover_sticker_id: sticker_pack.cover_sticker_id,
            description: sticker_pack.description,
            id: sticker_pack.id,
            name: sticker_pack.name,
            sku_id: sticker_pack.sku_id,
            sticker_ids,
        }
    }
}
