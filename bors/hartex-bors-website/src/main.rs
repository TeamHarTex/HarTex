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

//! # The website for bors.
//!
//! The website is home to the "bors cheatsheet" as well as the queues for certain repositories.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(lazy_cell)]
#![feature(let_chains)]

use std::sync::LazyLock;

use async_lock::OnceCell;
use handlebars::Handlebars;
use hartex_bors_database::client::SeaORMDatabaseClient;
use hartex_log::log;
use miette::IntoDiagnostic;
use rocket::config::Config;
use rocket::routes;

mod index;
mod queue;

pub(crate) static DATABASE: OnceCell<SeaORMDatabaseClient> = OnceCell::new();

pub(crate) static HANDLEBARS: LazyLock<Handlebars> = LazyLock::new(|| {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .register_templates_directory(".hbs", "./bors/hartex-bors-website/templates")
        .unwrap();

    handlebars
});

/// The entry point.
#[allow(clippy::no_effect_underscore_binding)]
#[rocket::main]
pub async fn main() -> miette::Result<()> {
    hartex_log::initialize();

    log::trace!("initializing handlebars instance");
    LazyLock::force(&HANDLEBARS);

    log::trace!("initializing database");
    DATABASE
        .get_or_init(|| async {
            SeaORMDatabaseClient::new(
                hartex_bors_database::initialize_database(false)
                    .await
                    .unwrap(),
            )
        })
        .await;

    log::debug!("igniting rocket");
    let rocket = rocket::custom(Config::figment().merge(("port", 9000)))
        .mount("/", routes![index::index, queue::queue])
        .ignite()
        .await
        .into_diagnostic()?;

    log::debug!("launching rocket");
    rocket.launch().await.into_diagnostic()?;

    Ok(())
}
