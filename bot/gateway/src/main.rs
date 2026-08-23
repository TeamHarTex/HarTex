/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use std::env;

use config::{Config, File};
use regex::regex;
use tracing::subscriber;

use crate::{boot::Settings, error::GatewayResult, gateway::GatewayRunner};

mod boot;
mod command;
mod error;
mod gateway;
mod shard;

#[tokio::main]
async fn main() -> GatewayResult<()> {
    subscriber::set_global_default(shared_tracing::subscriber())?;

    let config = Config::builder()
        .add_source(File::with_name("boot.settings.yml"))
        .build()?;
    let settings = config.try_deserialize::<Settings>()?;
    let token = regex!(r#"<%= ENV\[\"(.*)\"\] %>"#)
        .captures(&settings.token)
        .map_or_else(
            || Ok(settings.token.to_string()),
            |captures| env::var(&captures[0]),
        )?;

    let (gateway, _) = GatewayRunner::new(token).await?;
    tokio::spawn(async move { gateway.run().await });

    Ok(())
}
