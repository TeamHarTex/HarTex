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

use base::discord::model::application::command::CommandType;
use base::error::Result;
use clap::ArgMatches;
use ext::discord::model::application::command::HarTexCommand;
use futures_util::TryFutureExt;
use hyper::Client;

use crate::utils;

pub async fn create_cmd(matches: &ArgMatches) -> Result<()> {
    let name = matches.get_one::<String>("COMMAND_NAME").unwrap();
    let r#type = matches.get_one::<String>("COMMAND_TYPE").unwrap();

    let mut command = HarTexCommand {
        name: name.clone(),
        r#type: CommandType::from(r#type.parse::<u8>()?),
        ..Default::default()
    };

    let Some(loadbal_port) = matches.get_one::<String>("PORT") else {
        println!("create-cmd: load balancer port is missing");
        return Ok(());
    };

    command
        .description
        .replace(utils::prompt("Command description?")?);
    command
        .dm_permission
        .replace(utils::prompt("Enable command for DM?")?);

    println!("create-cmd: sending request");
    tokio::spawn(
        Client::new()
            .request(
                restreq::create_global_application_command::create_global_application_command(
                    command,
                    loadbal_port.parse::<u16>()?,
                )?,
            )
            .inspect_err(|error| println!("create-cmd: failed to send request: {error}")),
    );

    Ok(())
}
