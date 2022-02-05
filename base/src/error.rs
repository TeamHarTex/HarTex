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

//! An error type for error handling in the bot.

use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
        Result as FmtResult
    },
    io::Error as IoError,
    str::Utf8Error
};

use base64::DecodeError;
use ctrlc::Error as CtrlcError;
use toml::de::Error as TomlDeserializationError;

use crate::{
    discord::{
        embed_builder::{
            image_source::ImageSourceUrlError,
            EmbedError
        },
        gateway::{
            cluster::{
                ClusterCommandError,
                ClusterStartError
            },
            shard::SessionInactiveError
        },
        http::{
            error::Error as HttpError,
            request::{
                application::InteractionError,
                channel::message::{
                    create_message::CreateMessageError,
                    update_message::UpdateMessageError
                },
                guild::member::update_guild_member::UpdateGuildMemberError
            },
            response::DeserializeBodyError
        },
        model::gateway::payload::outgoing::update_presence::UpdatePresenceError
    },
    time::ParseError
};

/// Various error types used within HarTex.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum HarTexError {
    /// An error occurred whilst decoding base64 data.
    Base64DecodeError { error: DecodeError },
    /// An error occurred whilst sending a command to the gateway cluster.
    ClusterCommandError { error: ClusterCommandError },
    /// An error occurred whilst attempting to start up the gateway cluster.
    ClusterStartError { error: ClusterStartError },
    /// An error occurred whilst attempting to send a message.
    CreateMessageError { error: CreateMessageError },
    /// An error occurred whilst attempting to set the Ctrl-C handler.
    CtrlcError { error: CtrlcError },
    /// An error occurred whilst attempting to deserialize a JSON response payload.
    DeserializeBodyError { error: DeserializeBodyError },
    /// An error occurred whilst attempting to build a Discord embed.
    EmbedError { error: EmbedError },
    /// An error occurred whilst attempting to add an image source uri field to a Discord embed.
    EmbedImageSourceUrlError { error: ImageSourceUrlError },
    /// An error occurred whilst attempting to create an interaction.
    InteractionError { error: InteractionError },
    /// An I/O error.
    IoError { error: IoError },
    /// An error occurred whilst attempting to parse a timestamp.
    ParseError { error: ParseError },
    /// Signifies that a shard has not yet been started.
    SessionInactiveError { error: SessionInactiveError },
    /// An error occurred whilst attempting to deserialize a TOML object.
    TomlDeserializationError { error: TomlDeserializationError },
    /// An error occurred whilst making an HTTP request.
    TwilightHttpError { error: HttpError },
    /// An error occurred whilst attempting to update a guild member.
    UpdateGuildMemberError { error: UpdateGuildMemberError },
    /// An error occurred whilst attempting to update a message.
    UpdateMessageError { error: UpdateMessageError },
    /// An error occurred whilst attempting to update the self presence on Discord.
    UpdatePresenceError { error: UpdatePresenceError },
    /// An error occurred whilst attempting to validate a string encoded in UTF-8.
    Utf8ValidationError { error: Utf8Error },
    /// A custom error with a custom message.
    Custom { message: String }
}

impl Display for HarTexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(self, f)
    }
}

impl Error for HarTexError {}

impl From<ClusterCommandError> for HarTexError {
    fn from(error: ClusterCommandError) -> Self {
        Self::ClusterCommandError {
            error
        }
    }
}

impl From<ClusterStartError> for HarTexError {
    fn from(error: ClusterStartError) -> Self {
        Self::ClusterStartError {
            error
        }
    }
}

impl From<CreateMessageError> for HarTexError {
    fn from(error: CreateMessageError) -> Self {
        Self::CreateMessageError {
            error
        }
    }
}

impl From<CtrlcError> for HarTexError {
    fn from(error: CtrlcError) -> Self {
        Self::CtrlcError {
            error
        }
    }
}

impl From<DecodeError> for HarTexError {
    fn from(error: DecodeError) -> Self {
        Self::Base64DecodeError {
            error
        }
    }
}

impl From<DeserializeBodyError> for HarTexError {
    fn from(error: DeserializeBodyError) -> Self {
        Self::DeserializeBodyError {
            error
        }
    }
}

impl From<EmbedError> for HarTexError {
    fn from(error: EmbedError) -> Self {
        Self::EmbedError {
            error
        }
    }
}

impl From<HttpError> for HarTexError {
    fn from(error: HttpError) -> Self {
        Self::TwilightHttpError {
            error
        }
    }
}

impl From<ImageSourceUrlError> for HarTexError {
    fn from(error: ImageSourceUrlError) -> Self {
        Self::EmbedImageSourceUrlError {
            error
        }
    }
}

impl From<InteractionError> for HarTexError {
    fn from(error: InteractionError) -> Self {
        Self::InteractionError {
            error
        }
    }
}

impl From<IoError> for HarTexError {
    fn from(error: IoError) -> Self {
        Self::IoError {
            error
        }
    }
}

impl From<ParseError> for HarTexError {
    fn from(error: ParseError) -> Self {
        Self::ParseError {
            error
        }
    }
}

impl From<SessionInactiveError> for HarTexError {
    fn from(error: SessionInactiveError) -> Self {
        Self::SessionInactiveError {
            error
        }
    }
}

impl From<TomlDeserializationError> for HarTexError {
    fn from(error: TomlDeserializationError) -> Self {
        Self::TomlDeserializationError {
            error
        }
    }
}

impl From<UpdateGuildMemberError> for HarTexError {
    fn from(error: UpdateGuildMemberError) -> Self {
        Self::UpdateGuildMemberError {
            error
        }
    }
}

impl From<UpdateMessageError> for HarTexError {
    fn from(error: UpdateMessageError) -> Self {
        Self::UpdateMessageError {
            error
        }
    }
}

impl From<UpdatePresenceError> for HarTexError {
    fn from(error: UpdatePresenceError) -> Self {
        Self::UpdatePresenceError {
            error
        }
    }
}

impl From<Utf8Error> for HarTexError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8ValidationError {
            error
        }
    }
}

/// A global type-alias for handling the [`Result`] type throughout the codebase.
///
/// [`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
pub type HarTexResult<T> = Result<T, HarTexError>;
