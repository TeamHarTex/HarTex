//! # The `tz` Module
//!
//! This module contains the `Timezone` configuration model.

use std::fmt::Formatter;

use serde::de::{
    Error,
    Visitor
};

/// # Enum `Timezone`
///
/// Represents a timezone.
pub enum Timezone {
    /// # Enum Variant `Timezone::UTC`
    ///
    /// The UTC timezone.
    UTC
}

/// # Struct `TimezoneDeserializeStringVisitor`
///
/// A `String` visitor for deserializing a `Timezone`.
pub struct TimezoneDeserializeStringVisitor;

impl<'visitor> Visitor<'visitor> for TimezoneDeserializeStringVisitor {
    type Value = Timezone;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a string representing a timezone")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where E: Error {
        Ok(match v.as_str() {
            "UTC" => Timezone::UTC,
            _ => return Err(Error::custom("invalid timezone"))
        })
    }
}