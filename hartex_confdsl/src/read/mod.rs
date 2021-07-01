//! # The `read` Module
//!
//! This module defines utility traits for reading the Configuration DSL files.

use hartex_core::error::HarTexResult;

use crate::read::util::Ref;

mod sealed {
    /// # Trait `SealedTrait`
    ///
    /// Prevents the implementation of a trait outside this crate, when that trait implements
    /// this trait.
    pub trait SealedTrait { }
}
mod util;

pub mod slice;

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
pub trait Read<'deserialize>: sealed::SealedTrait {
    fn discard(&mut self);
    fn next(&mut self) -> HarTexResult<Option<u8>>;
    fn peek(&mut self) -> HarTexResult<Option<u8>>;

    fn pos(&self) -> Pos;
    fn peek_pos(&self) -> Pos;

    fn byte_offset(&mut self) -> usize;

    fn parse_str<'refstr>(&'refstr mut self, scratch: &'refstr mut Vec<u8>) -> HarTexResult<Ref<'deserialize, 'refstr, str>>;

    unsafe fn parse_str_unchecked<'refstr>(&'refstr mut self, scratch: &'refstr mut Vec<u8>) -> HarTexResult<Ref<'deserialize, 'refstr, [u8]>>;
}
