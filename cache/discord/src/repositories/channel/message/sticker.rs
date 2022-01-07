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

//! # The `sticker` Module
//!
//! This module contains the channel message sticker repository trait.

use hartex_base::discord::model::channel::message::sticker::{
    StickerId,
    StickerPackId
};
use hartex_cache_base::repository::{
    GetEntityFuture,
    Repository,
    StreamEntitiesFuture
};

use crate::{
    backend::DiscordBackend,
    entities::channel::message::sticker::{
        StickerEntity,
        StickerPackEntity
    }
};

/// # Trait `StickerPackRepository`
///
/// A repository containing sticker pack objects.
#[allow(clippy::module_name_repetitions)]
pub trait StickerPackRepository<B: DiscordBackend>: Repository<StickerPackEntity, B> {
    /// # Trait Method `stickers`
    ///
    /// Returns the stickers in this sticker pack.
    #[rustfmt::skip] // rustfmt suggests very shit formatting here, so we skip this
    fn stickers(&self, pack_id: StickerPackId)
        -> StreamEntitiesFuture<'_, StickerEntity, B::Error>;
}

/// # Trait `StickerRepository`
///
/// A repository containing sticker objects.
#[allow(clippy::module_name_repetitions)]
pub trait StickerRepository<B: Backend>: Repository<StickerEntity, B> {
    /// # Trait Method `sticker_pack`
    ///
    /// Returns the sticker pack associated with this sticker, if any.
    fn sticker_pack(
        &self,
        sticker_id: StickerId
    ) -> GetEntityFuture<'_, StickerPackEntity, B::Error>;
}
