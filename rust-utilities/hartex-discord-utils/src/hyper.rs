/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

use std::sync::Arc;

use hartex_discord_core::tokio::net::TcpStream;
use miette::IntoDiagnostic;
use tokio_rustls::client::TlsStream;
use tokio_rustls::rustls::pki_types::ServerName;
use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::rustls::RootCertStore;
use tokio_rustls::TlsConnector;
use webpki_roots::TLS_SERVER_ROOTS;

#[allow(clippy::missing_errors_doc)]
pub async fn tls_stream() -> miette::Result<TlsStream<TcpStream>> {
    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.extend(TLS_SERVER_ROOTS.iter().cloned());
    let tls_config = Arc::new(
        ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth(),
    );

    let connector = TlsConnector::from(tls_config);
    let dns_name = ServerName::try_from("discord.com").into_diagnostic()?;

    let stream = TcpStream::connect("discord.com:443")
        .await
        .into_diagnostic()?;

    connector.connect(dns_name, stream).await.into_diagnostic()
}
