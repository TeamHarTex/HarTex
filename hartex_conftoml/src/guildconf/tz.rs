//! # The `tz` Module
//!
//! This module contains the `Timezone` configuration model.

use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult
};

use serde::de::{
    Error,
    Visitor
};

/// # Enum `Timezone`
///
/// Represents a timezone.
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Converts the timezone to a offset usable with `chrono` timezones.
    #[allow(clippy::erasing_op)]
    #[must_use]
    pub fn into_offset_secs(self) -> i32 {
        match self {
            Self::UTC => 0 * 3600,
            Self::AsiaHongKong => 8 * 3600
        }
    }
}

impl Display for Timezone {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match *self {
            Self::AsiaHongKong => "Asia/Hong_Kong",
            Self::UTC => "UTC"
        })
    }
}

/// # Struct `GuildConfigTimezoneDeserializerRefStrVisitor`
///
/// A `&str` visitor for deserializing a `Timezone` for `GuildConfig`.
pub struct GuildConfigTimezoneDeserializerRefStrVisitor;

impl<'visitor> Visitor<'visitor> for GuildConfigTimezoneDeserializerRefStrVisitor {
    type Value = Timezone;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "a string representing a timezone")
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
