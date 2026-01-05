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

use std::net::SocketAddr;

use clap::Parser;
use color_eyre::Result;
use hartex_discord_grpc::manager::manager_server::ManagerServer;
use hartex_tracing::{self, eyre};
use rustls::crypto::aws_lc_rs;
use tonic::transport::Server;
use tracing::subscriber;
use twilight_http::Client;

use crate::{args::ManagerCliArgs, server::ManagerServerImpl};

mod args;
mod server;
mod state;

#[tokio::main]
pub async fn main() -> Result<()> {
    hartex_termios_utils::no_echoctl();
    eyre::initialize_eyre()?;
    subscriber::set_global_default(hartex_tracing::subscriber())?;

    aws_lc_rs::default_provider().install_default().unwrap();

    let args = ManagerCliArgs::parse();
    let port = args.port();

    let ip = if args.exposed() {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    }
    .parse()?;
    let addr = SocketAddr::new(ip, port);

    tracing::info!("{}", hartex_version::version());

    tracing::trace!("loading configuration from environment variables");
    let config = hartex_discord_envconf::load_configuration()?;

    tracing::trace!("obtaining suggested gateway connection configuration from Discord");
    let http = Client::new(config.token().to_owned());
    let info = http.gateway().authed().await?.model().await?;

    let current_user = http.current_user().await?.model().await?;

    tracing::info!("starting gRPC server on {addr}");
    Server::builder()
        .serve(
            addr,
            ManagerServer::new(ManagerServerImpl::new(info, current_user)),
        )
        .await?;

    Ok(())
}
