//! # The `permlvl` Module
//!
//! This module contains configuration models for permission levels.

pub mod roles;

// [PermissionLevels.Roles]
// RoleId1 = <perm id>
// RoleId2 = <perm id>
//
// [PermissionLevels.Users]
// UserId1 = <perm id>

//! # Struct `PermissionLevels`
//!
//! Represents the permission levels configured.
pub struct PermissionLevels {
    pub Roles: roles::PermissionLevelsRoles
}
