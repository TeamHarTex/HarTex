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
 * with HarTex; if not, If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `error` Module
//!
//! This module defines several types for error handling in the `HarTex` Discord bot.

use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
        Result as FmtResult
    },
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

/// # Enum `HarTexError`
///
/// An enumeration representing the various error types used within `HarTex`.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum HarTexError {
    /// # Enum Variant `HarTexError::Base64DecodeError`
    ///
    /// A wrapper around `base64::DecodeError`
    ///
    /// ## Fields
    /// - `error`, type `DecodeError`: the error returned when attempting to decode base64.
    Base64DecodeError { error: DecodeError },

    /// # Enum Variant `HarTexError::ClusterCommandError`
    ///
    /// A wrapper around `twilight_gateway::cluster::ClusterCommandError`.
    ///
    /// ## Fields
    /// - `error`, type: `ClusterCommandError`: the cluster command error when "commanding" the
    ///                                         cluster.
    ClusterCommandError { error: ClusterCommandError },

    /// # Enum Variant HarTexError::ClusterStartError
    ///
    /// A wrapper around `twilight_gateway::cluster::ClusterStartError`.
    ///
    /// ## Fields
    /// - `error`, type `ClusterStartError`: the cluster start error returned when building the
    ///                                      cluster.
    ClusterStartError { error: ClusterStartError },

    /// # Enum Variant `HarTexError::CreateMessageError`
    ///
    /// A wrapper around `twilight_http::request::channel::message::create_message::CreateMessageError`.
    ///
    /// ## Fields
    /// - `error`, type `CreateMessageError`: the error returned when attempting to send a message,
    CreateMessageError { error: CreateMessageError },

    /// # Enum Variant `HarTexError::CtrlcError`
    ///
    /// A wrapper around `ctrlc::Error`.
    ///
    /// ## Fields
    /// - `error`, type `Error`: the ctrlc error returned when setting the ctrl-c handler.
    CtrlcError { error: CtrlcError },

    /// # Enum Variant `HarTexError::DeserializeBodyError`
    ///
    /// A wrapper around `twilight_http::response::DeserializeBodyError`
    ///
    /// ## Fields
    /// - `error`, type `DeserializeBodyError`: the error returned when attempting to deserialize
    ///                                         an http response.
    DeserializeBodyError { error: DeserializeBodyError },

    /// # Enum Variant `HarTexError::EmbedError`
    ///
    /// A wrapper around `twilight_embed_builder::EmbedError`.
    ///
    /// ## Fields
    /// - `error`, type `EmbedError`: the embed error returned when building a Discord embed.
    EmbedError { error: EmbedError },

    /// # Enum Variant `HarTexError::EmbedImageSourceUrlError`
    ///
    /// A wrapper around `twilight_embed_builder::image_source::ImageSourceUrlError`.
    ///
    /// ## Fields
    /// - `error`, type `ImageSourceUrlError`: the error returned when trying to set a url for any
    ///                                        embed property.
    EmbedImageSourceUrlError { error: ImageSourceUrlError },

    /// # Enum Variant `HarTexError::InteractionError`
    ///
    /// A wrapper around `twilight_http::request::application::InteractionError`
    ///
    /// - `error`, type `InteractionError`: the error returned when attempting to register
    ///                                     an interaction.
    InteractionError { error: InteractionError },

    /// # Enum Variant `HarTexError::ParseError`
    ///
    /// A wrapper around `chrono::ParseError`
    ///
    /// - `error`, type `ParseError`: the datetime parsing error
    ParseError { error: ParseError },

    /// # Enum Variant `HarTexError::SessionInactiveError`
    ///
    /// A wrapper around `twilight_gateway::shard::SessionInactiveError`
    ///
    /// - `error`, type `SessionInactiveError`: the error returned when attempting to get
    ///                                         information about a shard.
    SessionInactiveError { error: SessionInactiveError },

    /// # Enum Variant `TomlDeserializationError`
    ///
    /// A wrapper around `toml::de::Error`
    ///
    /// ## Fields
    ///
    /// - `error`, type `Error`: the TOML deserialization error returned when attempting to
    ///                          deserializing TOML.
    TomlDeserializationError { error: TomlDeserializationError },

    /// # Enum Variant `HarTexError::TwilightHttpError`
    ///
    /// A wrapper around `twilight_http::error::Error`.
    ///
    /// ## Fields
    /// - `error`, type `Error`: the error returned when executing an HTTP request.
    TwilightHttpError { error: HttpError },

    /// # Enum Variant `HarTexError::UpdateGuildMemberError`
    ///
    /// A wrapper around `twilight_http::request::guild::member::update_guild_member::UpdateGuildMemberError`.
    ///
    /// ## Fields
    /// - `error`, type `UpdateGuildMemberError`: the error returned when attempting to update
    ///                                           a guild member.
    UpdateGuildMemberError { error: UpdateGuildMemberError },

    /// # Enum Variant `HarTexError::UpdatePresenceError`
    ///
    /// A wrapper around `twilight_http::request::channel::message::update_message::UpdateMessageError`.
    ///
    /// ## Fields
    /// - `error`, type `UpdateMessageError`: the message update error returned when
    ///                                       attempting to update a message.
    UpdateMessageError { error: UpdateMessageError },

    /// # Enum Variant `HarTexError::UpdatePresenceError`
    ///
    /// A wrapper around `twilight_model::gateway::payload::update_presence::UpdatePresenceError`.
    ///
    /// ## Fields
    /// - `error`, type `UpdatePresenceError`: the presence update error returned when
    ///                                        attempting to update the presence.
    UpdatePresenceError { error: UpdatePresenceError },

    /// # Enum Variant `HarTexError::Utf8ValidationError`
    ///
    /// A wrapper around `std::string::FromUtf8Error`.
    ///
    /// ## Fields
    /// - `error`, type `FromUtf8Error`: the error returned when attempting to construct a string
    ///                                  with a `Vec<u8>` with the UTF-8 encoding.
    Utf8ValidationError { error: Utf8Error },

    /// # Enum Variant `HarTexError::Custom`
    ///
    /// Represents a custom error that cannot be represented with any other variants of this
    /// enumeration.
    ///
    /// ## Fields
    /// - `message`, type `&str`: the error message.
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

impl From<Utf8Error> for HarTexError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8ValidationError {
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

/// # Type Alias `HarTexResult<T>`
///
/// A type alias for `Result<T, HarTexError>`, used for error-handling.
pub type HarTexResult<T> = Result<T, HarTexError>;
