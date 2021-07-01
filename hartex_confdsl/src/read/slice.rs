//! # The `slice` Module
//! 
//! This module defines a reader that operates on a slice of bytes.

/// # Struct `SliceRead`
///
/// A DSL input source that reads from a slice of bytes.
pub struct SliceRead<'bytes> {
    slice: &'bytes [u8],
    index: usize
}
