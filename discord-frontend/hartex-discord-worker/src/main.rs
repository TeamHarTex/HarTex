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

use hartex_discord_core::dotenv;
use hartex_discord_core::log;
use hartex_discord_core::tokio;
use lapin::{Connection, ConnectionProperties};
use lapin::message::DeliveryResult;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> hartex_discord_eyre::Result<()> {
    hartex_discord_eyre::initialize()?;
    log::initialize();

    log::trace!("loading environment variables");
    dotenv::dotenv()?;

    let username = std::env::var("GATEWAY_RABBITMQ_USERNAME")?;
    let password = std::env::var("GATEWAY_RABBITMQ_PASSWORD")?;
    let host = std::env::var("RABBITMQ_HOST")?;
    let port = std::env::var("RABBITMQ_PORT")?;
    let uri = format!("amqp://{username}:{password}@{host}:{port}");
    let uri_log = format!("amqp://{username}:<redacted>@{host}:{port}");

    log::trace!("creating rabbitmq amqp connection (uri: {})", &uri_log);
    let amqp_connection = Connection::connect(&uri, ConnectionProperties::default()).await?;

    let channel_inbound = amqp_connection.create_channel().await?;
    channel_inbound.basic_consume("gateway.inbound", "consumer", BasicConsumeOptions::default(), FieldTable::default())
        .await?
        .set_delegate(move |result: DeliveryResult| async move {
            if let Ok(Some(delivery)) = result {
                delivery.ack(BasicAckOptions::default()).await.expect("failed to ack");
                log::trace!("{}", String::from_utf8(delivery.data).unwrap());
            }
        });

    Ok(())
}
