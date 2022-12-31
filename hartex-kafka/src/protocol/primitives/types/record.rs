/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

use std::io::{ErrorKind, Read};

use super::super::errors::PrimitiveReadError;
use super::super::errors::RecordReadError;
use super::super::traits::PrimitiveRead;
use super::super::traits::RecordRead;
use super::super::types::Int16;
use super::super::types::Int8;
use super::super::types::VarInt;
use super::super::types::VarLong;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordBatchRecords {
    ControlBatch(RecordBatchControlBatch),
    Records(Vec<RecordBatchRecord>),
}

impl<R: Read> RecordRead<R> for RecordBatchRecords {
    fn read(reader: &mut R, is_control_batch: bool) -> Result<Self, RecordReadError> {
        if is_control_batch {
            return Ok(Self::ControlBatch(RecordBatchControlBatch::read(reader)?));
        }

        let mut records = Vec::new();

        loop {
            let result = VarInt::read(reader);

            if let Err(PrimitiveReadError::Io(err)) = &result && err.kind() == ErrorKind::UnexpectedEof {
                return Ok(Self::Records(records));
            } else if result.is_err() {
                return Err(result.map_err(RecordReadError::from).unwrap_err());
            }

            records.push(RecordBatchRecord::of_length(result.unwrap(), reader)?);
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordBatchControlBatch {
    pub kind: ControlBatchKind,
}

impl<R: Read> PrimitiveRead<R> for RecordBatchControlBatch {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let version = Int16::read(reader)?.0;
        if version != 0 {
            return Err(PrimitiveReadError::Generic(
                format!("unexpected version: {version}").into(),
            ));
        }

        let kind = match Int16::read(reader)?.0 {
            0 => ControlBatchKind::Abort,
            1 => ControlBatchKind::Commit,
            other => {
                return Err(PrimitiveReadError::Generic(
                    format!("unexpected control batch type: {other}").into(),
                ))
            }
        };

        Ok(Self { kind })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControlBatchKind {
    Abort,
    Commit,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordBatchRecord {
    pub attributes: Int8,
    pub key_length: VarInt,
    pub length: VarInt,
    pub offset_delta: VarInt,
    pub timestamp_delta: VarLong,
}

impl RecordBatchRecord {
    #[allow(clippy::missing_errors_doc)]
    pub fn of_length<R: Read>(
        length: VarInt,
        reader: &mut R,
    ) -> Result<Self, PrimitiveReadError> {
        let _ = length.0;
        let attributes = Int8::read(reader)?;
        let timestamp_delta = VarLong::read(reader)?;
        let offset_delta = VarInt::read(reader)?;
        let key_length = VarInt::read(reader)?;

        Ok(Self {
            attributes,
            key_length,
            length,
            offset_delta,
            timestamp_delta,
        })
    }
}
