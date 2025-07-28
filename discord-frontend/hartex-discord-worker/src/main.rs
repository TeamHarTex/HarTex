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

//! # Worker Process
//!
//! The worker process is the process thatr receives messages from the leader.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(incomplete_features)]
#![feature(deref_patterns)]
#![feature(map_try_insert)]

use hartex_discord_core::{discord::cache::DefaultInMemoryCache, dotenvy, tokio, tokio::signal};
use hartex_discord_grpc_protos::gateway::{
    GATEWAY_GRPC_FILE_DESCRIPTOR_SET, gateway_server::GatewayServer,
};
use miette::IntoDiagnostic;
use mimalloc::MiMalloc;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

use crate::grpc::GatewayWorkerServer;

mod errorhandler;
mod eventcallback;
mod grpc;
mod interaction;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Entry point.
#[allow(clippy::large_futures)]
#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> miette::Result<()> {
    tracing::subscriber::set_global_default(hartex_tracing::subscriber()).unwrap();

    hartex_tracing::trace!("loading environment variables");
    dotenvy::dotenv().into_diagnostic()?;

    hartex_tracing::trace!("initializing cache");
    let cache = DefaultInMemoryCache::new();

    let reflect = Builder::configure()
        .include_reflection_service(false)
        .register_encoded_file_descriptor_set(GATEWAY_GRPC_FILE_DESCRIPTOR_SET)
        .build_v1()
        .into_diagnostic()?;

    let addr = "127.0.0.1:6553".parse().unwrap();
    hartex_tracing::trace!("starting gRPC server, listing on {addr}");
    let service = GatewayServer::new(GatewayWorkerServer::new(cache));
    Server::builder()
        .add_service(service)
        .add_service(reflect)
        .serve(addr)
        .await
        .into_diagnostic()?;

    signal::ctrl_c().await.into_diagnostic()?;
    hartex_tracing::warn!("ctrl-c signal received, shutting down");

    Ok(())
}
