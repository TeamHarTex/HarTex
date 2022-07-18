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

use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn main() -> io::Result<()> {
    let buildconf_toml = Path::new("buildconf.toml");

    println!("configure: checking for existing buildconf.toml");
    let mut file = if buildconf_toml.exists() {
        println!("configure: a buildconf.toml file already exists. this configure program is only used for first-time configuration");

        println!("configure: initial configuration has already been done before.");
        println!("configure: refer to buildconf.toml.example for more configuration keys");

        println!("configure: run `./out/make` to build the bot");
        return Ok(());
    }
    else {
        File::create(buildconf_toml)?
    };

    println!("configure: using default cache backend: postgres");
    println!("configure: using default load balancer configuration:");
    println!("           - 1 server of type `rest`, host `127.0.0.1`,  listening on port `8000`");

    file.write_all(br#"[cache]
backend = "postgres"

[loadbal]
servers = [
    { type = "rest", address = "127.0.0.1:8000" }
]"#)?;

    println!("configure: initial configuration done.");
    println!("configure: refer to buildconf.toml.example for more configuration keys");

    println!("configure: run `./out/make` to build the bot");

    Ok(())
}
