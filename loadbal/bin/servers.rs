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
 *
 */

//! Service to get servers to load balance from the environment.

use std::lazy::SyncLazy;
use std::str::FromStr;

use dashmap::DashMap;
use hyper::Uri;

pub static SERVERS: SyncLazy<DashMap<String, Vec<Uri>>> = SyncLazy::new(|| {
    let map = DashMap::new();

    let servers = env!("LOADBAL_SERVERS");
    let mut servers = servers.split(';');

    for server in servers.by_ref() {
        if server.is_empty() {
            break;
        }

        let parts = server
            .split('-')
            .map(String::from)
            .collect::<Vec<String>>();
        let uri = Uri::from_str(&parts[1]).unwrap();
        if map.contains_key(&parts[0]) {
            map.alter(&parts[0], |_, mut vec: Vec<Uri>| {
                vec.push(uri);
                vec
            });
        } else {
            map.insert(parts[0].to_string(), vec![uri]);
        }
    }

    map
});

pub fn init() {
    SyncLazy::force(&SERVERS);
}
