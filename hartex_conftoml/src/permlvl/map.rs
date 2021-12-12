/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex; if not, If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `map` Module
//!
//! This module contains configuration models for permission levels.

use std::{
    fmt::{
        Formatter,
        Result as FmtResult
    },
    hash::Hash,
    marker::PhantomData
};

use dashmap::DashMap;
use serde::{
    de::{
        self,
        MapAccess
    },
    Deserialize
};

/// # Struct `PermissionLevelMap`
///
/// Represents a permission level map over an `Id` generic parameter.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct PermissionLevelMap<Id: Clone + Eq + Hash> {
    #[allow(dead_code)]
    pub map: DashMap<Id, u8>
}

impl<Id: Clone + Eq + Hash> Default for PermissionLevelMap<Id> {
    fn default() -> Self {
        Self {
            map: DashMap::default()
        }
    }
}

impl<Id: Clone + Eq + Hash> PartialEq for PermissionLevelMap<Id> {
    fn eq(&self, other: &Self) -> bool {
        self.map
            .iter()
            .all(|entry| other.map.get(entry.key()).unwrap().value() == entry.value())
    }
}

impl<'visitor, Id: Clone + Deserialize<'visitor> + Eq + Hash> Deserialize<'visitor>
    for PermissionLevelMap<Id>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'visitor> {
        deserializer.deserialize_map(PermissionLevelsPermissionLevelMapDeserializerMapVisitor {
            phantom: PhantomData,
            phantom2: PhantomData
        })
    }
}

pub struct PermissionLevelsPermissionLevelMapDeserializerMapVisitor<
    'visitor,
    Id: Clone + Deserialize<'visitor> + Eq + Hash
> {
    phantom: PhantomData<&'visitor ()>,
    phantom2: PhantomData<Id>
}

impl<'visitor, Id: Clone + Deserialize<'visitor> + Eq + Hash> de::Visitor<'visitor>
    for PermissionLevelsPermissionLevelMapDeserializerMapVisitor<'visitor, Id>
{
    type Value = PermissionLevelMap<Id>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "a map representing the permission levels of various ids")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'visitor> {
        let dashmap = DashMap::<Id, u8>::new();

        while let Some((key, value)) = access.next_entry()? {
            dashmap.insert(key, value);
        }

        Ok(PermissionLevelMap {
            map: dashmap
        })
    }
}
