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
use super::super::traits::RecordRead;
use super::record::RecordBatchRecords;
use super::Boolean;
use super::Int16;
use super::Int32;
use super::Int64;
use super::Int8;
use crate::blockvec::BlockVec;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordBatch {
    pub attributes: RecordBatchAttributes,
    pub base_offset: Int64,
    pub base_sequence: Int32,
    pub base_timestamp: Int64,
    pub batch_length: Int32,
    pub last_offset_delta: Int32,
    pub max_timestamp: Int64,
    pub partition_leader_epoch: Int32,
    pub producer_epoch: Int16,
    pub producer_id: Int64,
    pub records: RecordBatchRecords,
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

        let attributes = RecordBatchAttributes::read(reader)?;

        let last_offset_delta = Int32::read(reader)?;
        let base_timestamp = Int64::read(reader)?;
        let max_timestamp = Int64::read(reader)?;
        let producer_id = Int64::read(reader)?;
        let producer_epoch = Int16::read(reader)?;
        let base_sequence = Int32::read(reader)?;

        let records =
            RecordBatchRecords::read(reader, attributes.is_control_batch.0).map_err(|error| {
                PrimitiveReadError::Generic(format!("failed to read records: {error:?}").into())
            })?;

        Ok(Self {
            attributes,
            base_offset,
            base_sequence,
            base_timestamp,
            batch_length,
            last_offset_delta,
            max_timestamp,
            partition_leader_epoch,
            producer_epoch,
            producer_id,
            records,
        })
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordBatchAttributes {
    pub compression_type: CompressionType,
    pub has_delete_horizon_ms: Boolean,
    pub is_control_batch: Boolean,
    pub is_transactional: Boolean,
    pub timestamp_type: TimestampType,
}

impl<R: Read> PrimitiveRead<R> for RecordBatchAttributes {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let attributes = Int16::read(reader)?.0;
        let compression_type = match attributes & 0x7 {
            0 => CompressionType::None,
            1 => CompressionType::Gzip,
            2 => CompressionType::Snappy,
            3 => CompressionType::Lz4,
            4 => CompressionType::Zstd,
            other => {
                return Err(PrimitiveReadError::Generic(
                    format!("invalid compression type: {other}").into(),
                ))
            }
        };

        let timestamp_type = if (attributes >> 3) & 0x1 == 0 {
            TimestampType::CreateTime
        } else {
            TimestampType::LogAppendTime
        };

        let is_transactional = Boolean((attributes >> 4) & 0x1 == 1);
        let is_control_batch = Boolean((attributes >> 5) & 0x1 == 1);
        let has_delete_horizon_ms = Boolean((attributes >> 6) & 0x1 == 1);

        Ok(Self {
            compression_type,
            has_delete_horizon_ms,
            is_control_batch,
            is_transactional,
            timestamp_type,
        })
    }
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
