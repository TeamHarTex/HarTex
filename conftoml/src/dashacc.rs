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
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! # The `dashacc` Module
//!
//! This module contains configuration models specifically for dashboard access configuration.

use std::fmt::{
    Formatter,
    Result as FmtResult
};

use hartex_base::discord::model::id::UserId;
use serde::{
    de::{
        self,
        Error,
        Visitor
    },
    Deserialize
};

/// # Struct `DashboardAccess`
///
/// Represents the dashboard access of a user.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct DashboardAccess {
    #[serde(deserialize_with = "deserialize_userId")]
    pub userId: UserId,
    pub accessLevel: u8
}

/// # Struct `DashboardAccessUserIdDeserializerU64Visitor`
///
/// A `u64` visitor for deserializing a `UserId` for `DashboardAccess`.
pub struct DashboardAccessUserIdDeserializerU64Visitor;

impl<'visitor> Visitor<'visitor> for DashboardAccessUserIdDeserializerU64Visitor {
    type Value = UserId;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "a non-zero integer representing a user id")
    }

    #[allow(clippy::cast_sign_loss)]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error {
        UserId::new(v as u64).ok_or_else(|| Error::custom("the user id cannot be zero"))
    }
}

fn deserialize_userId<'deserialize, D>(deserializer: D) -> Result<UserId, D::Error>
where
    D: de::Deserializer<'deserialize> {
    deserializer.deserialize_u64(DashboardAccessUserIdDeserializerU64Visitor)
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::Debug,
        num::NonZeroU64
    };

    use serde_test::Token;

    use super::{
        DashboardAccess,
        Deserialize,
        UserId
    };

    // FIXME: use macros maybe?
    const _: fn() = || {
        fn static_assert_impl_all<
            'deserialize,
            T: ?Sized + Clone + Debug + Deserialize<'deserialize> + PartialEq
        >() {
        }

        static_assert_impl_all::<DashboardAccess>();
    };

    #[test]
    fn test_dashacc_de() {
        serde_test::assert_de_tokens(
            &[
                DashboardAccess {
                    userId: UserId(NonZeroU64::new(1234567887654321).unwrap()),
                    accessLevel: 0
                }
            ],
            &[
                Token::Seq {
                    len: Some(1)
                },
                Token::Struct {
                    name: "DashboardAccess",
                    len: 2
                },
                Token::Str("userId"),
                Token::I64(1234567887654321),
                Token::Str("accessLevel"),
                Token::U8(0),
                Token::StructEnd,
                Token::SeqEnd
            ]
        );
    }
}
