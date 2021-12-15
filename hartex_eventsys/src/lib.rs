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
 * with HarTex; if not, see <https://www.gnu.org/licenses/>.
 */

//! # `hartex_eventsys` - A Custom Event System for `HarTex` Discord bot
//!
//! The `hartex_eventsys` crate contains a custom event system as well as several custom events
//! for the `HarTex` Discord bot.

#![deny(clippy::pedantic, warnings, unsafe_code)]

pub mod emitter;
pub mod events;
pub mod listener;
