//! # The `args` Module
//!
//! This module contains an implementation of a command-argument iterator

use std::str::CharIndices;

/// # Struct `CommandArgs`
///
/// An iterator over the arguments of a command.
#[derive(Clone)]
pub struct CommandArgs<'a> {
    buffer: &'a str,
    indices: CharIndices<'a>,
    index: usize
}

impl<'a> CommandArgs<'a> {
    /// # Constructor `CommandArgs::new`
    ///
    /// Creates a new `CommandArgs` with a `&str`.
    ///
    /// ## Parameters
    /// - `refstr`, type `&str`: the reference to a string to create the `CommandArgs` for.
    pub fn new(refstr: &'a str) -> Self {
        Self::from(refstr)
    }
}

impl<'a> From<&'a str> for CommandArgs<'a> {
    fn from(refstr: &'a str) -> Self {
        let buffer = refstr.trim();

        Self {
            buffer,
            indices: buffer.char_indices(),
            index: 0
        }
    }
}
