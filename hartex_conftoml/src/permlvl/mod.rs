/*
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

//! # The `permlvl` Module
//!
//! This module contains configuration models for permission levels.

use std::{
    fmt::{
        Formatter,
        Result as FmtResult
    },
    num::NonZeroU64
};

use serde::{
    de::{
        Error,
        Visitor
    },
    Deserialize,
    Deserializer
};

pub mod map;

/// # Struct `PermissionLevels`
///
/// Represents the permission levels configured.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct PermissionLevels {
    #[serde(default)]
    pub roles: map::PermissionLevelMap<GenericId>,
    #[serde(default)]
    pub users: map::PermissionLevelMap<GenericId>
}

/// # Struct `RoleId`
///
/// Represents a role ID.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GenericId(NonZeroU64);

impl<'deserialize> Deserialize<'deserialize> for GenericId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'deserialize> {
        deserializer.deserialize_str(PermissionLevelsRolesMapGenericIdDeserializerRefstrVisitor)
    }
}

pub struct PermissionLevelsRolesMapGenericIdDeserializerRefstrVisitor;

impl<'visitor> Visitor<'visitor> for PermissionLevelsRolesMapGenericIdDeserializerRefstrVisitor {
    type Value = GenericId;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "a string representing a generic discord id")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error {
        let res = v.parse::<u64>();
        if res.is_err() {
            return Err(Error::custom("invalid integer"));
        }

        let nonzero_uint = NonZeroU64::new(res.unwrap());
        if nonzero_uint.is_none() {
            return Err(Error::custom("role id must not be zero"));
        }

        Ok(GenericId(nonzero_uint.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::Debug,
        num::NonZeroU64
    };

    use dashmap::DashMap;
    use serde_test::Token;

    use super::{
        map::PermissionLevelMap,
        Deserialize,
        GenericId,
        PermissionLevels
    };

    const _: fn() = || {
        fn static_assert_impl_all<
            'deserialize,
            T: ?Sized + Clone + Debug + Deserialize<'deserialize>
        >() {
        }

        static_assert_impl_all::<PermissionLevels>();
    };

    #[test]
    fn test_dashacc_de() {
        let dashmap_roles = DashMap::new();
        dashmap_roles.insert(GenericId(NonZeroU64::new(1234567887654321).unwrap()), 100);
        dashmap_roles.insert(GenericId(NonZeroU64::new(2345678998765432).unwrap()), 90);
        dashmap_roles.insert(GenericId(NonZeroU64::new(3456789009876543).unwrap()), 80);
        dashmap_roles.insert(GenericId(NonZeroU64::new(9876543223456789).unwrap()), 50);
        dashmap_roles.insert(GenericId(NonZeroU64::new(8765432112345678).unwrap()), 10);

        let dashmap_users = DashMap::new();
        dashmap_users.insert(GenericId(NonZeroU64::new(1000000000000001).unwrap()), 100);
        dashmap_users.insert(GenericId(NonZeroU64::new(2000000000000002).unwrap()), 90);

        serde_test::assert_de_tokens(
            &PermissionLevels {
                roles: PermissionLevelMap {
                    map: dashmap_roles
                },
                users: PermissionLevelMap {
                    map: dashmap_users
                }
            },
            &[
                Token::Struct {
                    name: "PermissionLevels",
                    len: 5
                },
                Token::Str("roles"),
                Token::Map {
                    len: Some(5)
                },
                Token::Str("1234567887654321"),
                Token::I64(100),
                Token::Str("2345678998765432"),
                Token::I64(90),
                Token::Str("3456789009876543"),
                Token::I64(80),
                Token::Str("9876543223456789"),
                Token::I64(50),
                Token::Str("8765432112345678"),
                Token::I64(10),
                Token::MapEnd,
                Token::Str("users"),
                Token::Map {
                    len: Some(2)
                },
                Token::Str("1000000000000001"),
                Token::I64(100),
                Token::Str("2000000000000002"),
                Token::I64(90),
                Token::MapEnd,
                Token::StructEnd
            ]
        );
    }
}
