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

use super::traits::PrimitiveRead;
use super::traits::PrimitiveWrite;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Boolean(pub bool);

impl<R: Read> PrimitiveRead<R> for Boolean {
    fn read(reader: &mut R) -> Result<Self, super::errors::PrimitiveReadError> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;

        Ok(Self(match buffer[0] {
            0 => false,
            _ => true,
        }))
    }
}

impl<W: Write> PrimitiveWrite<W> for Boolean {
    fn write(&self, writer: &mut W) -> Result<(), super::errors::PrimitiveWriteError> {
        Ok(writer.write_all(&[self.0.into()])?)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Int8(pub i8);

impl<R: Read> PrimitiveRead<R> for Int8 {
    fn read(reader: &mut R) -> Result<Self, super::errors::PrimitiveReadError> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;

        Ok(Self(i8::from_be_bytes(buffer)))
    }
}

impl<W: Write> PrimitiveWrite<W> for Int8 {
    fn write(&self, writer: &mut W) -> Result<(), super::errors::PrimitiveWriteError> {
        let buf = self.0.to_be_bytes();
        writer.write_all(&buf)?;
        
        Ok(())
    }
}
