/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
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

use std::{
    env,
    sync::{LazyLock, OnceLock},
};

use color_eyre::{
    Result,
    eyre::{WrapErr, eyre},
};
use twilight_http::Client;

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .token(TOKEN.get().unwrap().clone())
        .build()
});

pub static TOKEN: OnceLock<String> = OnceLock::new();

pub fn initialize_env() -> Result<()> {
    let token = env::var("TOKEN").wrap_err("environment variable `TOKEN` is not set")?;
    TOKEN
        .set(token)
        .map_err(|_| eyre!("TOKEN has already been initialized"))?;

    Ok(())
}
