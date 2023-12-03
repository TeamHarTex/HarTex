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

use std::collections::HashSet;
use std::env;
use std::str;

use hartex_backend_models::Response;
use hartex_backend_models_v2::bors::RepositoryPermissionsResponse;
use hartex_bors_core::models::GithubRepositoryName;
use hartex_log::log;
use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::client::conn::http2::handshake;
use hyper::header::ACCEPT;
use hyper::Method;
use hyper::Request;
use hyper_util::rt::TokioExecutor;
use miette::IntoDiagnostic;
use miette::Report;
use tokio::net::TcpStream;

use crate::Permission;

/// User permissions data structure.
#[allow(clippy::module_name_repetitions)]
pub struct UserPermissions {
    try_build_users: HashSet<String>,
    approve_users: HashSet<String>,
}

impl UserPermissions {
    /// Checks whether a user has a certain permission.
    #[must_use]
    pub fn user_has_permission(&self, username: &str, permission: Permission) -> bool {
        match permission {
            Permission::Approve => self.approve_users.contains(username),
            Permission::TryBuild => self.try_build_users.contains(username),
            _ => false,
        }
    }
}

pub(crate) async fn load(repository: &GithubRepositoryName) -> miette::Result<UserPermissions> {
    let try_build_users =
        load_permissions_from_api(repository.repository(), Permission::TryBuild).await?;
    let approve_users =
        load_permissions_from_api(repository.repository(), Permission::Approve).await?;

    Ok(UserPermissions {
        try_build_users,
        approve_users,
    })
}

async fn load_permissions_from_api(
    repository_name: &str,
    permission: Permission,
) -> miette::Result<HashSet<String>> {
    let api_domain = env::var("API_DOMAIN").into_diagnostic()?;
    let uri = format!(
        "http://{api_domain}/api/v1/bors/repositories/{repository_name}/permissions/{permission}"
    );

    log::debug!("sending a request to {}", &uri);

    let stream = TcpStream::connect(api_domain).await?;
    let (mut sender, connection) = handshake(TokioExecutor::new(), stream).await?;

    let request = Request::builder()
        .uri(uri)
        .header(ACCEPT, "application/json")
        .body(Empty::<Bytes>::new())
        .into_diagnostic()?;

    let result = sender.send_request(request).await.into_diagnostic()?;
    let body = result.collect().await.into_diagnostic()?.aggregate();

    let data: Response<RepositoryPermissionsResponse> = serde_json::from_reader(body.reader()).into_diagnostic()?;

    Ok(HashSet::from_iter(data.github_users().to_vec()))
}
