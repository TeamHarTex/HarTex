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

//! # The `guild` Module
//!
//! This module contains entities related to Discord guilds.

use hartex_base::discord::model::id::GuildId;

use crate::{
    backend::Backend,
    entities::guild::GuildEntity,
    repository::Repository
};

pub mod role;

// TODO: add `afk_channel` method to retrieve the guild AFK channel
// TODO: add `channel_ids` method to stream guild channel ids
// TODO: add `channels` method to stream guild channels
// TODO: add `emoji_ids` method stream guild emoji ids
// TODO: add `emojis` method to stream guild emojis
// TODO: add `member_ids` method to stream guild member (user) ids
// TODO: add `members` method to stream guild members
// TODO: add `presence_ids` method to stream guild presence ides
// TODO: add `presences` method to stream guild presences
// TODO: add `role_ids` method to stream guild role ids
// TODO: add `roles` method to stream guild roles
// TODO: add `rules_channel` method to retrieve the guild rules channel
// TODO: add `stage_instance_ids` method to stream guild stage instance ids
// TODO: add `stage_instances` method to stream guild stage instances
// TODO: add `sticker_ids` method to stream guild sticker ids
// TODO: add `stickers` method to stream guild stickers
// TODO: add `system_channel` method to retrieve the guild system channel
// TODO: add `thread_ids` method to stream guild thread ids
// TODO: add `threads` method to stream guild threads
// TODO: add `voice_state_user_ids` method to stream the user ids of voice states
// TODO: add `voice_states` method to stream voice states
// TODO: add `widget_channel` method to retrieve the widget channel
/// # Trait `GuildRepository`
///
/// A repository containing Discord guild objects.
#[allow(clippy::module_name_repetitions)]
pub trait GuildRepository<B: Backend>: Repository<GuildEntity, B> {}
