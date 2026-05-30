/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use std::net::SocketAddr;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "worker",
    about = "Worker instance of HarTex.",
    long_about = None
)]
pub struct WorkerCliArgs {
    #[arg(
        value_name = "CAP",
        default_value_t = 1,
        short,
        long,
        long_help = "maximum number of shards this worker is capable of running"
    )]
    capacity: u32,
    #[arg(
        value_name = "ADDR",
        default_value_t = SocketAddr::from(([127, 0, 0, 1], 3000)),
        short,
        long,
        long_help = "the address and port the worker instance manager listens on"
    )]
    manager: SocketAddr,
}

impl WorkerCliArgs {
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn manager_addr(&self) -> SocketAddr {
        self.manager
    }
}
