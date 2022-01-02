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

//! # The `nightly` Module
//!
//! This module contains configuration for opt-in nightly unstable features that are in the testing
//! phase.
//!
//! This API is very unstable and may change rapidly as features are added into the bot.

use serde::Deserialize;

/// # Struct `NightlyFeatures`
///
/// The opt-in nightly features that the bot provides.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct NightlyFeatures {
    // Experimental Support for the Discord Threads API
    #[serde(default = "default_feature_enabled")]
    pub threads: bool,
    // Experimental Support for Localization Facilities, i.e. timezones, languages
    #[serde(default = "default_feature_enabled")]
    pub localization: bool
}

#[must_use]
pub fn default_feature_enabled() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use serde_test::Token;

    use super::{
        Deserialize,
        NightlyFeatures
    };

    const _: fn() = || {
        fn static_assert_impl_all<
            'deserialize,
            T: ?Sized + Clone + Debug + Default + Deserialize<'deserialize> + PartialEq
        >() {
        }

        static_assert_impl_all::<NightlyFeatures>();
    };

    #[test]
    fn test_nightly_de() {
        serde_test::assert_de_tokens(
            &NightlyFeatures {
                threads: false,
                localization: true
            },
            &[
                Token::Struct {
                    name: "NightlyFeatures",
                    len: 2
                },
                Token::Str("threads"),
                Token::Bool(false),
                Token::Str("localization"),
                Token::Bool(true),
                Token::StructEnd
            ]
        );
    }
}
