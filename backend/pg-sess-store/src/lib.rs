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

use std::marker::PhantomData;
use std::time::Duration;

use rocket_session_store::{SessionResult, Store};

pub struct PgSessionStore<T> {
    phantom: PhantomData<T>,
}

impl<T> PgSessionStore<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

#[rocket::async_trait]
impl<T> Store for PgSessionStore<T> {
    type Value = T;

    async fn get(&self, id: &str) -> SessionResult<Option<Self::Value>> {
        todo!()
    }

    async fn set(&self, id: &str, value: Self::Value, duration: Duration) -> SessionResult<()> {
        todo!()
    }

    async fn touch(&self, id: &str, duration: Duration) -> SessionResult<()> {
        todo!()
    }

    async fn remove(&self, id: &str) -> SessionResult<()> {
        todo!()
    }
}
