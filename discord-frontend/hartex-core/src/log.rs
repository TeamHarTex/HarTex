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

pub use log::*;

use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn initialize() {
    let conf = Config::builder()
        .appender(
            Appender::builder().build(
                "stdout",
                Box::new(
                    ConsoleAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(
                            "{h([{d(%Y-%m-%d %H:%M:%S %Z)(local):>30}] [{l:>6}] [{M}]  {m})}{n}",
                        )))
                        .build(),
                ),
            ),
        )
        .logger(Logger::builder().build("amq_protocol_tcp", LevelFilter::Off))
        .logger(Logger::builder().build("async_io", LevelFilter::Off))
        .logger(Logger::builder().build("lapin", LevelFilter::Off))
        .logger(Logger::builder().build("mio", LevelFilter::Off))
        .logger(Logger::builder().build("pinky_swear", LevelFilter::Off))
        .logger(Logger::builder().build("polling", LevelFilter::Off))
        .logger(Logger::builder().build("rustls", LevelFilter::Off))
        .logger(Logger::builder().build("tokio_tungstenite", LevelFilter::Off))
        .logger(Logger::builder().build("tungstenite", LevelFilter::Off))
        .logger(Logger::builder().build("twilight_gateway", LevelFilter::Off))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .expect("failed to build log4rs configuration");

    log4rs::init_config(conf).expect("failed to initialize log4rs");
}
