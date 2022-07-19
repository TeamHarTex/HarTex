/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

#![deny(warnings)]

#[macro_use]
extern crate rocket;

use std::time::Duration;

use model::db::session::SessionEntry;
use pg_sess_store::PgSessionStore;
use rocket::{Build, Rocket};
use rocket_session_store::{CookieConfig, SessionStore};

use crate::routes::api_users_me::api_users_me;

mod routes;

#[rocket::main]
pub async fn main() {
    let store = PgSessionStore::<SessionEntry>::new();
    let session_store = SessionStore {
        store: Box::new(store),
        name: String::from("hartex-dashboard-session"),
        duration: Duration::from_secs(3600 * 24 * 3),
        cookie: CookieConfig {
            path: None,
            same_site: None,
            secure: false,
            http_only: true,
        },
    };

    rocket::build()
        .attach(session_store.fairing())
        .mount("/", routes![api_users_me])
        .launch()
        .await
        .unwrap();
}
