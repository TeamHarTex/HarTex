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
    /// # Enum Variant `Timezone::AsiaHongKong`
    ///
    /// The "Asia/Hong_Kong" timezone.
    AsiaHongKong,

    /// # Enum Variant `Timezone::UTC`
    ///
    /// The "UTC" timezone.
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

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: Error {
        Ok(match v {
            "Asia/Hong_Kong" => Timezone::AsiaHongKong,
            "UTC" => Timezone::UTC,
            _ => return Err(Error::custom("invalid timezone"))
        })
    }
}