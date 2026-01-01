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

use color_eyre::Result;
use hartex_discord_grpc::manager::manager_server::ManagerServer;
use hartex_tracing::{self, eyre};
use tonic::transport::Server;
use tracing::subscriber;

use crate::server::ManagerServerImpl;

mod server;

#[tokio::main]
pub async fn main() -> Result<()> {
    hartex_termios_utils::no_echoctl();
    eyre::initialize_eyre()?;
    subscriber::set_global_default(hartex_tracing::subscriber())?;

    // todo: allow port configuration in the command line
    Server::builder()
        .serve(
            "127.0.0.1:3000".parse()?,
            ManagerServer::new(ManagerServerImpl),
        )
        .await?;

    Ok(())
}
