/*
 * SPDX-License-Identifier: AGPL-3.0-only
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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct ConsumerError {
    pub kind: ConsumerErrorKind,
}

impl Display for ConsumerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            ConsumerErrorKind::InvalidGatewayPayload => f.write_str("invalid gateway payload"),
        }
    }
}

impl Error for ConsumerError {}

#[derive(Clone, Debug)]
pub enum ConsumerErrorKind {
    InvalidGatewayPayload,
}
