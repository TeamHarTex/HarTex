//! # The `args` Module
//!
//! This module contains an implementation of a command-argument iterator

use std::{
    fmt::{
        Debug,
        Formatter,
        Result as FmtResult
    },
    str::CharIndices
};

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

    /// # Instance Method `CommandArgs::into_remainder`
    ///
    /// Returns the remainder of the buffer that has not been parsed.
    ///
    /// If the string has been completely parsed, then this method returns `None`.
    pub fn into_remainder(self) -> Option<&'a str> {
        self.buffer.get(self.index..)
    }
}

impl<'a> Debug for CommandArgs<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("CommandArgs")
            .field("buffer", &self.buffer)
            .field("index", &self.index)
            .finish()
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
