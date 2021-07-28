//! # The `stopwatch` Module
//!
//! This module implements a stopwatch for timing things.

use std::convert::TryInto;

use hartex_core::time::{
    DateTime,
    Local
};

/// # Struct `Stopwatch`
///
/// A stopwatch for timing things.
pub struct Stopwatch {
    start: DateTime<Local>
}

impl Stopwatch {
    /// # Constructor `Stopwatch::new`
    ///
    /// Creates a new stopwatch.
    pub fn new() -> Self {
        Self {
            start: Local::now()
        }
    }

    pub fn elapsed_milliseconds(&self) -> u128 {
        let now = Local::now();

        (now - self.start).num_milliseconds().try_into().unwrap()
    }
}
