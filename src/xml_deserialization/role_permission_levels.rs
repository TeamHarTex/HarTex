///  Copyright 2020 - 2021 The HarTex Project Developers
///
///  Licensed under the Apache License, Version 2.0 (the "License");
///  you may not use this file except in compliance with the License.
///  You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
///  Unless required by applicable law or agreed to in writing, software
///  distributed under the License is distributed on an "AS IS" BASIS,
///  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
///  See the License for the specific language governing permissions and
///  limitations under the License.

extern crate serde;
extern crate quick_xml;

use std::{
    fmt::{
        Formatter,
        Result as FmtResult
    },
    marker::PhantomData
};

use serde::{
    de::{
        Deserialize,
        Deserializer,
        MapAccess,
        Visitor
    },
    ser::{
        Serialize,
        Serializer
    }
};

use super::RolePermissionLevel;

#[derive(Debug)]
crate struct RolePermissionLevels<K, V> {
    items: Vec<(K, V)>
}

impl<K: Eq, V> RolePermissionLevels<K, V> {
    crate fn new() -> Self {
        Self {
            items: Vec::<(K, V)>::new()
        }
    }

    crate fn get(&self, key: &K) -> Option<&V> {
        if let Some(pair) = self.items.iter().find(|&key_value_pair| {
            key_value_pair.0 == *key
        }) {
            Some(&pair.1)
        }
        else {
            None
        }
    }

    crate fn capacity(&self) -> usize {
        self.items.capacity()
    }

    crate fn contains_key(&self, key: &K) -> bool {
        self.items.iter().any(|(k, _)| {
            key == k
        })
    }

    crate fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.items.iter()
    }

    crate fn keys(&self) -> impl Iterator<Item = &K> {
        self.items.iter().map(|(key, _)| key)
    }

    crate fn values(&self) -> impl Iterator<Item = &V> {
        self.items.iter().map(|(_, value)| value)
    }

    crate fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    crate fn len(&self) -> usize {
        self.items.len()
    }

    crate fn insert(&mut self, key: K, value: V) {
        self.items.push((key, value));
    }
}

impl<K: Eq, V> Default for RolePermissionLevels<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Temporary<K, V> {
    #[serde(rename = "RolePermissionLevel")]
    vector: Vec<RolePermissionLevel<K, V>>
}

struct RolePermissionLevelsVisitor<K, V> {
    marker: PhantomData<RolePermissionLevels<K, V>>
}

impl<'deserialize, K: Eq, V> Visitor<'deserialize> for RolePermissionLevelsVisitor<K, V>
where
    K: Deserialize<'deserialize>,
    V: Deserialize<'deserialize> {
    type Value = RolePermissionLevels<K, V>;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("A RolePermissionLevels map.")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'deserialize> {
        let mut values = RolePermissionLevels::new();

        while let Some((key, value)) = access.next_entry()? {
            values.insert(key, value);
        }

        Ok(values)
    }
}

impl<'deserialize, K: Eq + Clone + Copy, V: Clone + Copy> Deserialize<'deserialize> for RolePermissionLevels<K, V>
where 
    K: Deserialize<'deserialize>,
    V: Deserialize<'deserialize> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'deserialize> {
        Ok(
            RolePermissionLevels {
                items: <Temporary<K, V> as Deserialize<'deserialize>>::deserialize(deserializer)?
                    .vector
                    .iter()
                    .map(|level|
                        (level.role_id, level.permission_integer))
                    .collect::<_>()
            }
        )
    }
}

impl<K: Eq + Clone + Copy, V: Clone + Copy> Serialize for RolePermissionLevels<K, V>
where
    K: Serialize,
    V: Serialize, {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        let mut vector = Vec::new();

        self.iter().for_each(|&(key, value)| {
            vector.push(RolePermissionLevel { role_id: key, permission_integer: value });
        });

        let temporary = Temporary { vector };

        temporary.serialize(serializer)
    }
}
