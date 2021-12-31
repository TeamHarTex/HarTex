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
//! This module contains the channel message sticker repository trait.

use crate::{
    backend::Backend,
    entities::channel::message::sticker::{
        StickerEntity,
        StickerPackEntity
    },
    repository::Repository
};

/// # Trait `StickerRepository`
///
/// A repository containing sticker objects.
#[allow(clippy::module_name_repetitions)]
pub trait StickerRepository<B: Backend>: Repository<StickerEntity, B> {}

pub trait StickerPackRepository<B: Backend>: Repository<StickerPackEntity, B> {}
