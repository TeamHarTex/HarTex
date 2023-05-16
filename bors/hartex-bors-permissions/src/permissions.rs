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
use hartex_backend_models_v1::bors::RepositoryPermissionsResponse;
use hartex_bors_github::models::GithubRepositoryName;
use hartex_eyre::eyre::Report;
use hartex_log::log;
use hyper::Client;
use hyper::Method;
use hyper::Request;
use hyper::body::HttpBody;
use hyper::header::ACCEPT;

use crate::Permission;

/// User permissions data structure.
pub struct UserPermissions {
    try_build_users: HashSet<String>,
}

impl UserPermissions {
    /// Checks whether a user has a certain permission.
    pub fn user_has_permission(&self, username: &str, permission: Permission) -> bool {
        match permission {
            Permission::TryBuild => self.try_build_users.contains(username),
            _ => false,
        }
    }
}

async fn load(repository: &GithubRepositoryName) -> hartex_eyre::Result<UserPermissions> {
    let try_build_users = load_permissions_from_api(repository.repository(), Permission::TryBuild).await?;

    Ok(UserPermissions {
        try_build_users,
    })
}

async fn load_permissions_from_api(
    repository_name: &str,
    permission: Permission
) -> hartex_eyre::Result<HashSet<String>> {
    let client = Client::builder().build_http::<String>();
    let api_domain = env::var("API_DOMAIN")?;
    let uri = format!("http://{api_domain}/api/v1/bors/repositories/{repository_name}/permissions/{permission}");

    log::debug!("sending a request to {}", &uri);

    let request = Request::builder()
        .uri(uri)
        .method(Method::GET)
        .header(ACCEPT, "application/json")
        .body(String::new())?;

    let mut response = client.request(request).await?;
    let mut full = String::new();
    while let Some(result) = response.body_mut().data().await {
        full.push_str(str::from_utf8(&result?)?);
    }
    if !response.status().is_success() {
        log::error!("unsuccessful HTTP request, response: {full}");

        return Err(Report::msg(format!(
            "unsuccessful HTTP request, with status code {}",
            response.status()
        )));
    }

    let response = serde_json::from_str::<Response<RepositoryPermissionsResponse>>(&full)?;
    let data = response.data();

    todo!()
}
