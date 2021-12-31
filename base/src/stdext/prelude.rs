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

//! # The `prelude` Module
//!
//! This module contains useful preludes for various types exported in the prelude of the Rust
//! Standard Library.

#[cfg(feature = "twilight-bundled")]
use crate::discord::model::{
    guild::PartialMember,
    id::UserId
};

/// # Trait `OptionExt`
///
/// Extensions for `Option<T>` in the Rust Standard Library.
pub trait OptionExt<'a> {
    /// # Trait Method `OptionExt::as_refstr`
    ///
    /// Converts the inner `String` of an `Option<T>` into a `&str`.
    fn as_refstr(&'a self) -> Option<&'a str> {
        None
    }

    /// # Trait Method `OptionExt::as_refstr`
    ///
    /// Maps the inner value (and unwraps the wrapping `Option<T>`)
    fn map_opt_user_id(&'a self) -> Option<UserId> {
        unimplemented!()
    }
}

#[cfg(feature = "twilight-bundled")]
impl<'a> OptionExt<'a> for Option<PartialMember> {
    fn map_opt_user_id(&'a self) -> Option<UserId> {
        if self.is_none() {
            return None;
        }

        let partial_member = self.clone().unwrap();
        partial_member.user.map(|user| user.id)
    }
}

impl<'a> OptionExt<'a> for Option<String> {
    fn as_refstr(&'a self) -> Option<&'a str> {
        self.as_ref().map(|string| &string[..])
    }
}
