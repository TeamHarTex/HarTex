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

use std::path::PathBuf;

use rocket::get;
use rocket::response::content::RawHtml;
use serde::Serialize;

#[derive(Serialize)]
struct QueueData {
    repository: String,
}

/// The endpoint returning the queue page.
#[get("/queue/<repository..>")]
pub async fn queue(repository: PathBuf) -> RawHtml<String> {
    let repository_string = repository.to_string_lossy().to_string();

    RawHtml(
        crate::HANDLEBARS
            .render(
                "queue",
                &QueueData {
                    repository: repository_string.replace("\\", "/"),
                },
            )
            .unwrap(),
    )
}
