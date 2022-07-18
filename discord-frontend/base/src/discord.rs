/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

//! Re-exports to the commonly referenced libraries from the Twilight Ecosystem.
//!
//! Each of the re-exported crates from the Twilight Ecosystem are gated behind a feature flag.
//!
//! Examples:
//!
//!  - `twilight-gateway` is gated behind the `discord-gateway` feature flag; and
//!  - `twilight-model` is gated behind the `discord-model` feature flag.
//!
//! All other re-exported crates follow a similar pattern, by replacing the `twilight` identifier
//! with `discord`.

#[cfg(feature = "discord-gateway")]
pub use twilight_gateway as gateway;

#[cfg(feature = "discord-http")]
pub use twilight_http as http;

#[cfg(feature = "discord-model")]
pub use twilight_model as model;
