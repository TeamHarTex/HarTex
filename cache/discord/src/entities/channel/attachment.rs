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

//! # The `attachment` Module
//!
//! This module implements the channel attachment entity.

use hartex_base::{
    discord::model::{
        channel::Attachment,
        id::{
            AttachmentId,
            MessageId
        }
    },
    stdext::prelude::*
};
use hartex_cache_base::entity::Entity;

/// # Struct `AttachmentEntity`
///
/// An attachment entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
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
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_refstr()
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_refstr()
    }

    #[must_use]
    pub fn ephemeral(&self) -> bool {
        self.ephemeral
    }

    #[must_use]
    pub fn filename(&self) -> &str {
        self.filename.as_ref()
    }

    #[must_use]
    pub fn height(&self) -> Option<u64> {
        self.height
    }

    #[must_use]
    pub fn proxy_url(&self) -> &str {
        self.proxy_url.as_str()
    }

    #[must_use]
    pub fn size(&self) -> u64 {
        self.size
    }

    #[must_use]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    #[must_use]
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

impl From<(MessageId, Attachment)> for AttachmentEntity {
    fn from((message_id, attachment): (MessageId, Attachment)) -> Self {
        Self {
            content_type: attachment.content_type,
            description: attachment.description,
            ephemeral: attachment.ephemeral,
            filename: attachment.filename,
            height: attachment.height,
            id: attachment.id,
            message_id,
            proxy_url: attachment.proxy_url,
            size: attachment.size,
            url: attachment.url,
            width: attachment.width
        }
    }
}
