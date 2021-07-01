//! # The `slice` Module
//!
//! This module defines a reader that operates on a slice of bytes.

use crate::read::{
    sealed,
    Pos
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

    fn pos_or_index(&self, index: usize) -> Pos {
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
}

impl<'bytes> sealed::SealedTrait for SliceRead<'bytes> { }
