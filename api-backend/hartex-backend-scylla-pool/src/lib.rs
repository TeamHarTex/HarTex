/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;

use rocket::figment::Figment;
use rocket_db_pools::Config;
use rocket_db_pools::Error;
use rocket_db_pools::Pool;
use scylla::frame::Compression;
use scylla::Session;
use scylla::SessionBuilder;
use scylla::transport::errors::NewSessionError;

pub struct ScyllaPool {
    url: String,
}

#[rocket::async_trait]
impl Pool for ScyllaPool {
    type Connection = Session;
    type Error = Error<NewSessionError>;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>()?;

        Ok(Self {
            url: config.url
        })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        let username = env::var("API_SCYLLADB_USERNAME").unwrap_or(String::new());
        let passwd = env::var("API_SCYLLADB_PASSWORD").unwrap_or(String::new());

        SessionBuilder::new()
            .known_node(&self.url)
            .compression(Some(Compression::Lz4))
            .user(username, passwd)
            .build()
            .await
            .map_err(Error::Get)
    }

    async fn close(&self) {}
}
