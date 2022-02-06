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
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! Useful extensions to the prelude of The Rust Standard Library.

use hartex_base::discord::model::{
    channel::thread::ThreadMember,
    guild::PartialMember,
    id::{marker::UserMarker, Id},
};

/// Extensions for [`Option<T>`] in The Rust Standard Library.
///
/// [`Option<T>`]: https://doc.rust-lang.org/nightly/std/option/enum.Option.html
pub trait OptionExt<'a> {
    /// Converts the inner [`String`] of an [`Option<T>`] into a [`&str`].
    ///
    /// [`&str`]: https://doc.rust-lang.org/nightly/std/primitive.str.html
    /// [`String`]: https://doc.rust-lang.org/nightly/std/string/struct.String.html
    /// [`Option<T>`]: https://doc.rust-lang.org/nightly/std/option/enum.Option.html
    fn as_refstr(&'a self) -> Option<&'a str> {
        None
    }

    /// Maps the inner value (and unwraps the wrapping [`Option<T>`]).
    ///
    /// [`Option<T>`]: https://doc.rust-lang.org/nightly/std/option/enum.Option.html
    fn map_opt_user_id(&'a self) -> Option<Id<UserMarker>> {
        unimplemented!()
    }
}

impl<'a> OptionExt<'a> for Option<PartialMember> {
    fn map_opt_user_id(&'a self) -> Option<Id<UserMarker>> {
        if self.is_none() {
            return None;
        }

        let partial_member = self.clone().unwrap();
        partial_member.user.map(|user| user.id)
    }
}

#[cfg(feature = "twilight-bundled")]
impl<'a> OptionExt<'a> for Option<ThreadMember> {
    fn map_opt_user_id(&'a self) -> Option<Id<UserMarker>> {
        if self.is_none() {
            return None;
        }

        let partial_member = self.clone().unwrap();
        partial_member.user_id
    }
}

impl<'a> OptionExt<'a> for Option<String> {
    fn as_refstr(&'a self) -> Option<&'a str> {
        self.as_ref().map(|string| &string[..])
    }
}
