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

/// # Struct `PermissionLevels`
///
/// Represents the permission levels configured.
pub struct PermissionLevels<'visitor> {
    pub Roles: map::PermissionLevelMap<'visitor, RoleId>
}
