//! # The `read` Module
//!
//! This module defines utility traits for reading the Configuration DSL files.

use hartex_core::error::HarTexResult;

mod sealed {
    /// # Trait `SealedTrait`
    ///
    /// Prevents the implementation of a trait outside this crate, when that trait implements
    /// this trait.
    pub trait SealedTrait { }
}
mod util;

/// # Struct `Position`
///
/// Represents a position in file.
struct Pos {
    pub line: usize,
    pub column: usize
}

/// # Trait `Read`
///
/// Trait that is used by the deserializer for iterating over the input.
pub trait Read<'de>: sealed::SealedTrait {
    fn discard(&mut self) -> HarTexResult<Option<u8>>;
    fn next(&mut self) -> HarTexResult<Option<u8>>;
    fn peek(&mut self) -> HarTexResult<Option<u8>>;

    fn pos(&mut self) -> Pos;
    fn peek_pos(&mut self) -> Pos;

    fn byte_offset(&mut self) -> usize;

}
