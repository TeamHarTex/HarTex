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

use std::any;
use std::mem;

#[derive(Debug)]
pub struct BlockVec<T> {
    blocks: Vec<Vec<T>>,
    per_block: usize,
    remaining: usize,
}

impl<T> BlockVec<T> {
    pub fn new(expected: usize) -> Self {
        Self::with_block_size(expected, 1024 * 1024 * 10)
    }

    pub fn with_block_size(expected: usize, block_size: usize) -> Self {
        let size = mem::size_of::<T>();
        let per_block = if size == 0 {
            expected
        } else {
            block_size / size
        };

        if per_block == 0 {
            panic!("insufficient block size for type {}", any::type_name::<T>());
        }

        Self {
            blocks: vec![Vec::with_capacity(per_block.min(expected))],
            per_block,
            remaining: expected,
        }
    }
}
