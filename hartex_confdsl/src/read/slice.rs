//! # The `slice` Module
//!
//! This module defines a reader that operates on a slice of bytes.

use std::{
    cmp,
    str
};

use hartex_core::error::{
    HarTexError,
    HarTexResult};

use crate::read::{
    sealed,
    util::Ref,
    Pos,
    Read
};

/// # Struct `SliceRead`
///
/// A DSL input source that reads from a slice of bytes.
pub struct SliceRead<'bytes> {
    slice: &'bytes [u8],
    index: usize
}

impl<'bytes> SliceRead<'bytes> {
    /// # Constructor `SliceRead::new`
    ///
    /// Creates a new `SliceRead` with the given slice of bytes.
    ///
    /// ## Parameters
    /// - `slice`, of type `&[u8]`: the slice of bytes
    pub fn new(slice: &'bytes [u8]) -> Self {
        Self {
            slice,
            index: 0
        }
    }

    fn pos_of_index(&self, index: usize) -> Pos {
        let mut pos = Pos {
            line: 1,
            column: 0
        };

        for refbyte in &self.slice[index..] {
            match *refbyte {
                b'\n' => {
                    pos.line += 1;
                    pos.column = 0;
                }
                _ => pos.column += 1
            }
        }

        pos
    }

    fn parse_str_bytes<'refstr, T, F>(&'refstr mut self, scratch: &'refstr mut Vec<u8>, validate: bool, res: F) -> HarTexResult<Ref<'bytes, 'refstr, T>>
    where
        T: ?Sized + 'refstr,
        F: for<'fnonce> FnOnce(&'refstr Self, &'fnonce [u8]) -> HarTexResult<&'fnonce T> {
        todo!()
    }
}

impl<'bytes> Read<'bytes> for SliceRead<'bytes> {
    fn discard(&mut self) {
        self.index += 1;
    }

    fn next(&mut self) -> HarTexResult<Option<u8>> {
        Ok(if self.index < self.slice.len() {
            let byte = self.slice[self.index];
            self.index += 1;

            Some(byte)
        }
        else {
            None
        })
    }

    fn peek(&mut self) -> HarTexResult<Option<u8>> {
        Ok(if self.index < self.slice.len() {
            Some(self.slice[self.index])
        }
        else {
            None
        })
    }

    fn pos(&self) -> Pos {
        self.pos_of_index(self.index)
    }

    fn peek_pos(&mut self) -> Pos {
        self.pos_of_index(cmp::min(self.slice.len(), self.index + 1))
    }

    fn byte_offset(&mut self) -> usize {
        self.index
    }

    fn parse_str<'refstr>(&'refstr mut self, scratch: &'refstr mut Vec<u8>) -> HarTexResult<Ref<'bytes, 'refstr, str>> {
        self.parse_str_bytes(scratch, true, as_str)
    }

    unsafe fn parse_str_unchecked<'refstr>(&'refstr mut self, scratch: &'refstr mut Vec<u8>) -> HarTexResult<Ref<'bytes, 'refstr, [u8]>> {
        self.parse_str_bytes(scratch, false, |_, bytes| Ok(bytes))
    }
}

fn as_str<'deserialize, 'refstr, R: Read<'deserialize>>(read: &R, slice: &'refstr [u8]) -> HarTexResult<&'refstr str> {
    str::from_utf8(slice).or_else(|_| {
        let pos = read.pos();

        Err(HarTexError::DslError {
            line: pos.line,
            column: pos.column,
            message: "invalid utf8 unicode code point"
        })
    })
}

impl<'bytes> sealed::SealedTrait for SliceRead<'bytes> { }
