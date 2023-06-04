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

//! # Bors Website - Index Page

use hartex_bors_core::DatabaseClient;
use rocket::get;
use rocket::response::content::RawHtml;
use serde::Serialize;

use crate::DATABASE;

#[derive(Serialize)]
struct IndexData {
    repositories: Vec<Repository>,
}

#[derive(Serialize)]
struct Repository {
    label: String,
}

#[get("/")]
pub async fn index() -> hartex_eyre::Result<RawHtml<String>> {
    let database = DATABASE.get().unwrap();
    let repositories = database.get_repositories().await?;

    Ok(RawHtml(
        crate::HANDLEBARS
            .render(
                "index",
                &IndexData {
                    repositories: repositories
                        .iter()
                        .map(|repository| Repository {
                            label: repository.name.clone(),
                        })
                        .collect(),
                },
            )
            .unwrap(),
    ))
}
