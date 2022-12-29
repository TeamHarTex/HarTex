/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

use std::any;
use std::io::Error;
use std::io::Read;
use std::mem;

#[derive(Debug)]
pub struct BlockVec<T> {
    blocks: Vec<Vec<T>>,
    per_block: usize,
    remaining: usize,
}

impl<T> BlockVec<T> {
    #[must_use]
    pub fn new(expected: usize) -> Self {
        Self::with_block_size(expected, 1024 * 1024 * 10)
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn with_block_size(expected: usize, block_size: usize) -> Self {
        let size = mem::size_of::<T>();
        let per_block = if size == 0 {
            expected
        } else {
            block_size / size
        };

        assert_ne!(
            per_block,
            0,
            "insufficient block size for type {}",
            any::type_name::<T>()
        );

        Self {
            blocks: vec![Vec::with_capacity(per_block.min(expected))],
            per_block,
            remaining: expected,
        }
    }
}

impl BlockVec<u8> {
    #[allow(clippy::missing_errors_doc)]
    pub fn read_exact<R: Read>(mut self, reader: &mut R) -> Result<Self, Error> {
        while self.remaining > 0 {
            let mut buffer = self
                .blocks
                .last_mut()
                .expect("at least one block is expected");
            if buffer.len() >= self.per_block {
                self.blocks
                    .push(Vec::with_capacity(self.remaining.min(self.per_block)));
                buffer = self.blocks.last_mut().expect("just pushed a new block");
            }

            let read_length = self.remaining.min(self.per_block - buffer.len());
            let buffer_start_position = buffer.len();
            buffer.resize(buffer.len() + read_length, 0);

            reader.read_exact(&mut buffer[buffer_start_position..])?;
            self.remaining -= read_length;
        }

        Ok(self)
    }
}

impl<T> From<BlockVec<T>> for Vec<T> {
    fn from(block_vec: BlockVec<T>) -> Self {
        if block_vec.blocks.len() == 1 {
            block_vec
                .blocks
                .into_iter()
                .next()
                .expect("number of blocks has been checked")
        } else {
            let mut result = Self::with_capacity(block_vec.blocks.iter().map(Self::len).sum());
            block_vec
                .blocks
                .into_iter()
                .for_each(|mut block| result.append(&mut block));

            result
        }
    }
}
