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

//! # The `attachment` Module
//!
//! This module implements the channel attachment entity.

use hartex_base::{
    discord::model::id::{
        AttachmentId,
        MessageId
    },
    stdext::prelude::*
};

use crate::entity::Entity;

/// # Struct `AttachmentEntity`
///
/// An attachment entity.
pub struct AttachmentEntity {
    content_type: Option<String>,
    description: Option<String>,
    ephemeral: bool,
    filename: String,
    height: Option<u64>,
    id: AttachmentId,
    message_id: MessageId,
    proxy_url: String,
    size: u64,
    url: String,
    width: Option<u64>
}

impl AttachmentEntity {
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_refstr()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_refstr()
    }

    pub fn ephemeral(&self) -> bool {
        self.ephemeral
    }

    pub fn filename(&self) -> &str {
        self.filename.as_ref()
    }

    pub fn height(&self) -> Option<u64> {
        self.height
    }

    pub fn proxy_url(&self) -> &str {
        self.proxy_url.as_str()
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn width(&self) -> Option<u64> {
        self.width
    }
}

impl Entity for AttachmentEntity {
    type Id = (MessageId, AttachmentId);

    fn id(&self) -> Self::Id {
        (self.message_id, self.id)
    }
}
