//! # The `permlvl` Module
//!
//! This module contains configuration models for permission levels.

// [PermissionLevels.Roles]
// RoleId1 = <perm id>
// RoleId2 = <perm id>
//
// [PermissionLevels.Users]
// UserId1 = <perm id>

pub mod map;

use hartex_core::discord::model::id::RoleId;
use serde::Deserialize;

/// # Struct `PermissionLevels`
///
/// Represents the permission levels configured.
#[derive(Deserialize)]
pub struct PermissionLevels {
    pub Roles: map::PermissionLevelMap<RoleId>
}

#[cfg(test)]
mod tests {
}
