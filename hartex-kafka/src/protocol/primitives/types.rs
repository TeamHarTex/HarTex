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
use std::string::String as StdString;

use integer_encoding::VarIntReader;
use integer_encoding::VarIntWriter;
use uuid::Uuid as UuidInner;

use super::errors::PrimitiveReadError;
use super::errors::PrimitiveWriteError;
use super::traits::PrimitiveRead;
use super::traits::PrimitiveWrite;
use crate::blockvec::BlockVec;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Boolean(pub bool);

impl<R: Read> PrimitiveRead<R> for Boolean {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;

        Ok(Self(match buffer[0] {
            0 => false,
            _ => true,
        }))
    }
}

impl<W: Write> PrimitiveWrite<W> for Boolean {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        Ok(writer.write_all(&[self.0.into()])?)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Int8(pub i8);

impl<R: Read> PrimitiveRead<R> for Int8 {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 1];
        reader.read_exact(&mut buffer)?;

        Ok(Self(i8::from_be_bytes(buffer)))
    }
}

impl<W: Write> PrimitiveWrite<W> for Int8 {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        let buf = self.0.to_be_bytes();
        writer.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Int16(pub i16);

impl<R: Read> PrimitiveRead<R> for Int16 {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 2];
        reader.read_exact(&mut buffer)?;

        Ok(Self(i16::from_be_bytes(buffer)))
    }
}

impl<W: Write> PrimitiveWrite<W> for Int16 {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        let buf = self.0.to_be_bytes();
        writer.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Int32(pub i32);

impl<R: Read> PrimitiveRead<R> for Int32 {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;

        Ok(Self(i32::from_be_bytes(buffer)))
    }
}

impl<W: Write> PrimitiveWrite<W> for Int32 {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        let buf = self.0.to_be_bytes();
        writer.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Int64(pub i64);

impl<R: Read> PrimitiveRead<R> for Int64 {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;

        Ok(Self(i64::from_be_bytes(buffer)))
    }
}

impl<W: Write> PrimitiveWrite<W> for Int64 {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        let buf = self.0.to_be_bytes();
        writer.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Uint32(pub u32);

impl<R: Read> PrimitiveRead<R> for Uint32 {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;

        Ok(Self(u32::from_be_bytes(buffer)))
    }
}

impl<W: Write> PrimitiveWrite<W> for Uint32 {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        let buf = self.0.to_be_bytes();
        writer.write_all(&buf)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VarInt(pub i32);

impl<R: Read> PrimitiveRead<R> for VarInt {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        Ok(Self(reader.read_varint()?))
    }
}

impl<W: Write> PrimitiveWrite<W> for VarInt {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        writer.write_varint(self.0)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VarLong(pub i64);

impl<R: Read> PrimitiveRead<R> for VarLong {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        Ok(Self(reader.read_varint()?))
    }
}

impl<W: Write> PrimitiveWrite<W> for VarLong {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        writer.write_varint(self.0)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UnsignedVarInt(pub u64);

impl<R: Read> PrimitiveRead<R> for UnsignedVarInt {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 1];
        let mut result = 0;
        let mut shift = 0;

        loop {
            reader.read_exact(&mut buffer)?;
            let last_group_flag: u64 = buffer[0].into();

            result |= (last_group_flag & 0x7F) << shift;
            shift += 7;

            if (last_group_flag & 0x80) == 0 {
                break;
            }

            if shift > 63 {
                return Err(PrimitiveReadError::Generic(
                    "overflow occurred while reading unsigned varint".into(),
                ));
            }
        }

        Ok(Self(result))
    }
}

impl<W: Write> PrimitiveWrite<W> for UnsignedVarInt {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        let mut current = self.0;

        loop {
            let mut group =
                u8::try_from(current & 0x7F).map_err(PrimitiveWriteError::IntOverflow)?;
            current >>= 7;

            if current > 0 {
                group |= 0x80;
            }

            writer.write_all(&[group])?;

            if current == 0 {
                break;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Uuid(pub UuidInner);

impl<R: Read> PrimitiveRead<R> for Uuid {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 16];
        reader.read_exact(&mut buffer)?;

        Ok(Self(UuidInner::from_bytes(buffer)))
    }
}

impl<W: Write> PrimitiveWrite<W> for Uuid {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        writer.write_all(&self.0.into_bytes())?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float64(pub f64);

impl<R: Read> PrimitiveRead<R> for Float64 {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let mut buffer = [0u8; 8];
        reader.read_exact(&mut buffer)?;

        Ok(Self(f64::from_be_bytes(buffer)))
    }
}

impl<W: Write> PrimitiveWrite<W> for Float64 {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        writer.write_all(&self.0.to_be_bytes())?;

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct String(pub StdString);

impl<R: Read> PrimitiveRead<R> for String {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let length = usize::try_from(Int16::read(reader)?.0)
            .map_err(|error| PrimitiveReadError::Generic(Box::new(error)))?;
        let mut buffer = BlockVec::new(length);
        buffer = buffer.read_exact(reader)?;

        Ok(Self(StdString::from_utf8(buffer.into()).map_err(
            |error| PrimitiveReadError::Generic(Box::new(error)),
        )?))
    }
}

impl<W: Write> PrimitiveWrite<W> for String {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        let length = i16::try_from(self.0.len()).map_err(PrimitiveWriteError::IntOverflow)?;
        Int16(length).write(writer)?;
        writer.write_all(self.0.as_bytes())?;

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CompactString(pub StdString);

impl<R: Read> PrimitiveRead<R> for CompactString {
    fn read(reader: &mut R) -> Result<Self, PrimitiveReadError> {
        let len = UnsignedVarInt::read(reader)?;
        match len.0 {
            0 => Err(PrimitiveReadError::Generic("CompactStrings must have non-zero length".into())),
            length => {
                let actual_length = usize::try_from(length)? - 1;

                let mut buffer = BlockVec::new(actual_length);
                buffer = buffer.read_exact(reader)?;

                Ok(Self(StdString::from_utf8(buffer.into()).map_err(|error| PrimitiveReadError::Generic(Box::new(error)))?))
            }
        }
    }
}

impl<W: Write> PrimitiveWrite<W> for CompactString {
    fn write(&self, writer: &mut W) -> Result<(), PrimitiveWriteError> {
        let length = u64::try_from(self.0.len() + 1).map_err(PrimitiveWriteError::IntOverflow)?;
        UnsignedVarInt(length).write(writer)?;
        writer.write_all(self.0.as_bytes())?;

        Ok(())
    }
}
