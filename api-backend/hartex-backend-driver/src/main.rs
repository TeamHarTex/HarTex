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

//! # Backend Driver
//!
//! This is the driver crate of the API backend for HarTex, uniting all components of the backend
//! and contains the core routing logic that routes requests to the corresponding request handlers.
//!
//! The driver also registers certain useful error catchers that return custom JSON payloads.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]

use std::env;
#[cfg(not(unix))]
use std::future;
use std::pin::pin;
use std::pin::Pin;
use std::time::Duration;

use axum::routing::post;
use axum::Router;
use dotenvy::Error;
use hartex_errors::dotenv;
use hartex_log::log;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::Request;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use miette::IntoDiagnostic;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::watch;
use tower_http::timeout::TimeoutLayer;
use tower_service::Service;

/// # Entry Point
///
/// This is the entry point of the API backend for HarTex. This does the heavy lifting of building
/// an Axum server and starting it.
#[allow(clippy::ignored_unit_patterns)]
#[allow(clippy::no_effect_underscore_binding)]
#[tokio::main]
pub async fn main() -> miette::Result<()> {
    hartex_log::initialize();

    log::trace!("loading environment variables");
    if let Err(error) = dotenvy::dotenv() {
        match error {
            Error::LineParse(content, index) => Err(dotenv::LineParseError {
                src: content,
                err_span: (index - 1, 1).into(),
            })?,
            _ => todo!(),
        }
    }

    log::debug!("starting axum server");
    let app = Router::new()
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .route(
            "/api/:version/stats/uptime",
            post(hartex_backend_routes::uptime::post_uptime),
        );

    let listener = TcpListener::bind(env::var("API_DOMAIN").into_diagnostic()?)
        .await
        .into_diagnostic()?;

    let (close_tx, close_rx) = watch::channel(());

    loop {
        let (socket, remote_addr) = tokio::select! {
            result = listener.accept() => {
                result.unwrap()
            }
            _ = shutdown() => {
                break;
            }
        };

        log::trace!("accepted connection from {remote_addr}");

        let service = app.clone();
        let close_rx = close_rx.clone();

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);

            let hyper_service =
                service_fn(move |request: Request<Incoming>| service.clone().call(request));

            let connection = hyper::server::conn::http2::Builder::new(TokioExecutor::new())
                .serve_connection(socket, hyper_service);
            let mut pinned: Pin<&mut _> = pin!(connection);
            loop {
                tokio::select! {
                    result = pinned.as_mut() => {
                        if result.is_err() {
                            log::error!("failed to serve connection, see error below");
                            println!("{}", result.into_diagnostic().unwrap_err());
                        }
                        break;
                    }
                    _ = shutdown() => {
                        log::trace!("signal received, starting graceful shutdown");
                        pinned.as_mut().graceful_shutdown();
                    }
                }
            }

            log::trace!("connection from {remote_addr} closed");

            drop(close_rx);
        });
    }

    drop(close_rx);
    drop(listener);

    log::trace!("waiting for {} tasks to finish", close_tx.receiver_count());
    close_tx.closed().await;

    Ok(())
}

#[allow(clippy::ignored_unit_patterns)]
async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
