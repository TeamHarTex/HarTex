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

//! # HarTex Load Balancer
//!
//! Implementation of load balancing for servers used by the codebase.

#![feature(let_else)]

use std::env as stdenv;

use base::cmdline;
use base::error::Result;
use base::logging;
use base::panicking;

#[tokio::main(flavor = "multi-thread")]
pub async fn main() -> Result<()> {
    logging::init();
    panicking::init();

    let args = stdenv::args().collect::<Vec<_>>();
    let args = &args[1..];
    let mut base_options = cmdline::Options::new();
    let options = base_options.reqopt(
        "",
        "port",
        "The port for the load balancer to run on",
        "PORT",
    );

    if args.is_empty() {
        loadbal_usage(options);
        return Ok(());
    }

    if let Err(error) = env::load() {
        log::error!("env error: {error}");
        log::warn!("environment variables cannot be loaded; exiting");
        log::info!("help: please make sure that the required environment variables are present");
        log::info!(r#"help: see the above errors (those that start with "retrieval failed")"#);

        return Ok(());
    }

    let result = options.parse(args);
    if let Err(error) = result {
        match error {
            cmdline::Fail::UnrecognizedOption(option) => {
                println!("gateway: unrecognized option: {option}");
            }
            cmdline::Fail::OptionMissing(option) => {
                println!("gateway: missing required option: {option}");
            }
            _ => (),
        }

        return Ok(());
    }
    let matches = result.unwrap();

    let Ok(Some(_port)) = matches.opt_get::<u16>("port") else {
        log::error!("could not parse port argument; exiting");

        return Ok(());
    };

    Ok(())
}

pub fn loadbal_usage(options: &mut cmdline::Options) {
    println!("{}", options.usage("Usage: loadbal [options] [args...]"));
}
