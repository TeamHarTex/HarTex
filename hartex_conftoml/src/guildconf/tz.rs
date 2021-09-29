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

impl Timezone {
    /// # Instance Method `into_offset_secs`
    ///
    /// Converts the timezone to a offset usable with chrono timezones.
    pub fn into_offset_secs(self) -> i32 {
        match self {
            Self::UTC => 0 * 3600,  // UTC+00:00
            Self::AsiaHongKong => 8 * 3600  // UTC+08:00
        }
    }
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
    where
        E: Error {
        Ok(match v {
            "Asia/Hong_Kong" => Timezone::AsiaHongKong,
            "UTC" => Timezone::UTC,
            _ => return Err(Error::custom("invalid timezone"))
        })
    }
}