//! # The `dashacc` Module
//!
//! This module contains configuration models specifically for dashboard access configuration.

use serde::Deserialize;

/// # Struct `DashboardAccess`
///
/// Represents the dashboard access of a user.
#[derive(Deserialize)]
pub struct DashboardAccess {
    pub userId: u64,
    pub accessLevel: u8
}
