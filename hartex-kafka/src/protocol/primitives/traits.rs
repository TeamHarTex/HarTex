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
use std::io::Write;

use super::errors::PrimitiveReadError;
use super::errors::PrimitiveWriteError;
use super::errors::RecordReadError;

pub trait PrimitiveRead<R: Read>: Sized {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError>;
}

pub trait PrimitiveWrite<W: Write>: Sized {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError>;
}

pub trait RecordRead<R: Read>: Sized {
    fn read(reader: &mut R, is_control_batch: bool) -> Result<Self, RecordReadError>;
}
