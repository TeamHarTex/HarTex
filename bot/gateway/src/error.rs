/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::{
    env::VarError,
    num::ParseIntError,
    process::{ExitCode, Termination},
};

use async_nats::{ConnectError, SubscribeError};
use config::ConfigError;
use prost::DecodeError;
use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;
use twilight_gateway::error::ChannelError;
use twilight_http::{Error as TwilightHttpError, response::DeserializeBodyError};

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("channel error: {0}")]
    ChannelError(#[from] ChannelError),
    #[error("configuration error: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("environment error: {0}")]
    EnvironmentError(#[from] VarError),
    #[error("body deserialization error: {0}")]
    JsonDeserializationError(#[from] DeserializeBodyError),
    #[error("NATS connection error: {0}")]
    NatsConnectionError(#[from] ConnectError),
    #[error("protobuf payload decode error: {0}")]
    NatsProtobufPayloadDecodeError(#[from] DecodeError),
    #[error("NATS subscriber error: {0}")]
    NatsSubscriberError(#[from] SubscribeError),
    #[error("parse int error: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("set global default error: {0}")]
    SetGlobalDefaultError(#[from] SetGlobalDefaultError),
    #[error("http error: {0}")]
    TwilightHttpError(#[from] TwilightHttpError),
}

pub type GatewayResult<T> = Result<T, GatewayError>;

pub struct GatewayTermination<T>(GatewayResult<T>);

impl<T> From<GatewayResult<T>> for GatewayTermination<T> {
    fn from(result: GatewayResult<T>) -> Self {
        GatewayTermination(result)
    }
}

impl<T: Termination> Termination for GatewayTermination<T> {
    fn report(self) -> ExitCode {
        match self.0 {
            Ok(value) => value.report(),
            Err(err) => {
                tracing::error!("{err}");
                ExitCode::FAILURE
            }
        }
    }
}
