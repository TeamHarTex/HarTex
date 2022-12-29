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

use std::io::Read;

use super::super::errors::PrimitiveReadError;
use super::super::errors::RecordReadError;
use super::super::traits::PrimitiveRead;
use super::super::traits::RecordRead;
use super::super::types::Int16;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordBatchRecords {
    ControlBatch(RecordBatchControlBatch),
    Records,
}

impl<R: Read> RecordRead<R> for RecordBatchRecords {
    fn read(reader: &mut R, is_control_batch: bool) -> Result<Self, RecordReadError> {
        if is_control_batch {
            return Ok(Self::ControlBatch(RecordBatchControlBatch::read(reader)?));
        }
        
        todo!()
    }
}

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
