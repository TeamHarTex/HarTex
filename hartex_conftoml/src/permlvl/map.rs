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
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PermissionLevelMap<Id: Clone + Eq + Hash> {
    #[allow(dead_code)]
    pub map: DashMap<Id, u8>
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
