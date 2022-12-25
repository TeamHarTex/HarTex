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
use super::super::traits::PrimitiveRead;
use super::Boolean;
use super::Int32;
use super::Int64;
use super::Int8;
use crate::blockvec::BlockVec;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordBatch {
    pub base_offset: Int64,
    pub batch_length: Int32,
    pub partition_leader_epoch: Int32,
}

impl<R: Read> PrimitiveRead<R> for RecordBatch {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let base_offset = Int64::read(reader)?;

        let batch_length = Int32::read(reader)?;
        let actual_length = usize::try_from(batch_length.0)
            .map_err(PrimitiveReadError::IntOverflow)?
            .checked_sub(9)
            .ok_or_else(|| {
                PrimitiveReadError::Generic(
                    format!("record batch length is too small: {}", batch_length.0).into(),
                )
            })?;

        let partition_leader_epoch = Int32::read(reader)?;

        let magic = Int8::read(reader)?.0;
        if magic != 2 {
            return Err(PrimitiveReadError::Generic(
                format!("invalid magic for record batch: {magic}").into(),
            ));
        }

        let crc = u32::from_be_bytes(Int32::read(reader)?.0.to_be_bytes());
        let mut data = BlockVec::new(actual_length);
        data = data.read_exact(reader)?;
        let data_vec: Vec<u8> = data.into();
        let actual_crc = crc32c::crc32c(&data_vec);

        if crc != actual_crc {
            return Err(PrimitiveReadError::Generic(
                format!("invalid crc32c: expected 0x{crc:x}, found 0x{actual_crc:x} instead")
                    .into(),
            ));
        }

        Ok(Self {
            base_offset,
            batch_length,
            partition_leader_epoch,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordBatchAttributes {
    pub compression_type: CompressionType,
    pub has_delete_horizon_ms: Boolean,
    pub is_transactional: Boolean,
    pub is_control_batch: Boolean,
    pub timestamp_type: TimestampType,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompressionType {
    Gzip,
    Lz4,
    None,
    Snappy,
    Zstd,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimestampType {
    CreateTime,
    LogAppendTime,
}
