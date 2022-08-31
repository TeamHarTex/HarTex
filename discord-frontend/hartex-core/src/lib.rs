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

//! # The HarTex Core Library
//!
//! The HarTex Core Library is the foundation of the HarTex. It defines the primitive building
//! blocks for all the components of HarTex.
//!
//! The HarTex Core Library is *minimal*: it is not aware of the implementation of the components
//! of HarTex, as they are out of scope of this crate.
//!
//! # Using the HarTex Core Library
//!
//! The HarTex Core Library mostly consists of feature flags that can be enabled when necessary.
//! Acknowledging the fact that not all usages of the HarTex Core Library requires *all* of the
//! exposed public API from it, the introduction of feature flags can reduce unncessary bloat
//! included for certain usages that may otherwise determine some components of the HarTex Core
//! Library being unncessary.
